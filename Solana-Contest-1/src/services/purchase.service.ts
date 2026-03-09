import prisma from "../utils/prisma";
import { AppError } from "../utils/errors";
import { isWhitelisted } from "./whitelist.service";
import {
    validateReferralCode,
    incrementUsedCount,
} from "./referral.service";

export const calculateTieredCost = (amount: number, tiers: any[], flatPrice: number) => {
    if (!tiers || tiers.length === 0) {
        return amount * flatPrice;
    }

    const sorted = [...tiers].sort((a, b) => a.minAmount - b.minAmount);
    let remaining = amount;
    let totalCost = 0;

    for (const tier of sorted) {
        if (remaining <= 0) break;
        const tierCapacity = tier.maxAmount - tier.minAmount;
        const filledFromTier = Math.min(remaining, tierCapacity);
        totalCost += filledFromTier * tier.pricePerToken;
        remaining -= filledFromTier;
    }

    if (remaining > 0) {
        totalCost += remaining * flatPrice;
    }

    return totalCost;
};

export const createPurchase = async (launchId: string, userId: string, data: any) => {
    const { walletAddress, amount, txSignature, referralCode } = data;

    const launch = await prisma.launch.findUnique({
        where: { id: launchId },
        include: { tiers: true },
    });
    if (!launch) throw new AppError("Launch not found", 404);

    const totalPurchasedAgg = await prisma.purchase.aggregate({
        where: { launchId },
        _sum: { amount: true },
    });
    const totalPurchased = totalPurchasedAgg._sum.amount || 0;

    const now = new Date();
    let status: string;
    if (totalPurchased >= launch.totalSupply) status = "SOLD_OUT";
    else if (now < new Date(launch.startsAt)) status = "UPCOMING";
    else if (now > new Date(launch.endsAt)) status = "ENDED";
    else status = "ACTIVE";

    if (status !== "ACTIVE") {
        throw new AppError(`Launch is ${status}, purchases not allowed`, 400);
    }

    const whitelisted = await isWhitelisted(launchId, walletAddress);
    if (!whitelisted) {
        throw new AppError("Wallet address not whitelisted", 400);
    }

    const userPurchasesAgg = await prisma.purchase.aggregate({
        where: { launchId, userId },
        _sum: { amount: true },
    });
    const userTotal = userPurchasesAgg._sum.amount || 0;

    if (userTotal + amount > launch.maxPerWallet) {
        throw new AppError(
            `Exceeds max per wallet limit. Current: ${userTotal}, Requested: ${amount}, Max: ${launch.maxPerWallet}`,
            400
        );
    }

    if (totalPurchased + amount > launch.totalSupply) {
        throw new AppError("Exceeds total supply", 400);
    }

    const existingTx = await prisma.purchase.findUnique({
        where: { txSignature },
    });
    if (existingTx) {
        throw new AppError("Duplicate transaction signature", 400);
    }

    let totalCost = calculateTieredCost(amount, launch.tiers, launch.pricePerToken);

    let referralCodeId: string | null = null;
    if (referralCode) {
        const referral = await validateReferralCode(launchId, referralCode);
        totalCost = totalCost * (1 - referral.discountPercent / 100);
        referralCodeId = referral.id;
        await incrementUsedCount(referral.id);
    }

    try {
        const purchase = await prisma.purchase.create({
            data: {
                launchId,
                userId,
                walletAddress,
                amount,
                totalCost,
                txSignature,
                referralCodeId,
            },
        });

        return purchase;
    } catch (err: any) {
        if (err.code === "P2002") {
            throw new AppError("Duplicate transaction signature", 400);
        }
        throw err;
    }
};

export const listPurchases = async (launchId: string, userId: string) => {
    const launch = await prisma.launch.findUnique({ where: { id: launchId } });
    if (!launch) throw new AppError("Launch not found", 404);

    const isCreator = launch.creatorId === userId;
    const where = isCreator ? { launchId } : { launchId, userId };

    const purchases = await prisma.purchase.findMany({
        where,
        orderBy: { createdAt: "desc" },
    });

    return { purchases, total: purchases.length };
};
