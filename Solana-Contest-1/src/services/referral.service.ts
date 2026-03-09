import prisma from "../utils/prisma";
import { AppError } from "../utils/errors";

export const createReferral = async (launchId: string, creatorId: string, data: any) => {
    const launch = await prisma.launch.findUnique({ where: { id: launchId } });
    if (!launch) throw new AppError("Launch not found", 404);
    if (launch.creatorId !== creatorId) {
        throw new AppError("Forbidden: not the launch creator", 403);
    }

    try {
        const referral = await prisma.referralCode.create({
            data: {
                launchId,
                code: data.code,
                discountPercent: data.discountPercent,
                maxUses: data.maxUses,
            },
        });
        return referral;
    } catch (err: any) {
        if (err.code === "P2002") {
            throw new AppError("Duplicate referral code for this launch", 409);
        }
        throw err;
    }
};

export const listReferrals = async (launchId: string, creatorId: string) => {
    const launch = await prisma.launch.findUnique({ where: { id: launchId } });
    if (!launch) throw new AppError("Launch not found", 404);
    if (launch.creatorId !== creatorId) {
        throw new AppError("Forbidden: not the launch creator", 403);
    }

    return prisma.referralCode.findMany({ where: { launchId } });
};

export const validateReferralCode = async (launchId: string, code: string) => {
    const referral = await prisma.referralCode.findUnique({
        where: { launchId_code: { launchId, code } },
    });

    if (!referral) throw new AppError("Invalid referral code", 400);
    if (referral.usedCount >= referral.maxUses) {
        throw new AppError("Referral code exhausted", 400);
    }

    return referral;
};

export const incrementUsedCount = async (referralId: string) => {
    return prisma.referralCode.update({
        where: { id: referralId },
        data: { usedCount: { increment: 1 } },
    });
};
