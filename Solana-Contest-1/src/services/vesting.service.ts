import prisma from "../utils/prisma";
import { AppError } from "../utils/errors";

export const getVestingSchedule = async (launchId: string, walletAddress: string) => {
    const launch = await prisma.launch.findUnique({
        where: { id: launchId },
        include: { vesting: true },
    });
    if (!launch) throw new AppError("Launch not found", 404);

    const purchasesAgg = await prisma.purchase.aggregate({
        where: { launchId, walletAddress },
        _sum: { amount: true },
    });
    const totalPurchased = purchasesAgg._sum.amount || 0;

    if (!launch.vesting) {
        return {
            totalPurchased,
            tgeAmount: totalPurchased,
            cliffEndsAt: null,
            vestedAmount: totalPurchased,
            lockedAmount: 0,
            claimableAmount: totalPurchased,
        };
    }

    const { cliffDays, vestingDays, tgePercent } = launch.vesting;
    const tgeAmount = Math.floor(totalPurchased * tgePercent / 100);
    const lockedTotal = totalPurchased - tgeAmount;

    const cliffEndsAt = new Date(launch.endsAt);
    cliffEndsAt.setDate(cliffEndsAt.getDate() + cliffDays);

    const now = new Date();

    if (now < cliffEndsAt) {
        return {
            totalPurchased,
            tgeAmount,
            cliffEndsAt: cliffEndsAt.toISOString(),
            vestedAmount: 0,
            lockedAmount: lockedTotal,
            claimableAmount: tgeAmount,
        };
    }

    const vestingStartMs = cliffEndsAt.getTime();
    const elapsed = Math.min(now.getTime() - vestingStartMs, vestingDays * 24 * 60 * 60 * 1000);
    const vestingFraction = vestingDays > 0 ? elapsed / (vestingDays * 24 * 60 * 60 * 1000) : 1;

    const vestedAmount = Math.floor(lockedTotal * vestingFraction);
    const lockedAmount = lockedTotal - vestedAmount;
    const claimableAmount = tgeAmount + vestedAmount;

    return {
        totalPurchased,
        tgeAmount,
        cliffEndsAt: cliffEndsAt.toISOString(),
        vestedAmount,
        lockedAmount,
        claimableAmount,
    };
};
