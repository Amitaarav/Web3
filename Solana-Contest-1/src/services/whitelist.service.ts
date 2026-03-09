import prisma from "../utils/prisma";
import { AppError } from "../utils/errors";

export const addAddresses = async (launchId: string, creatorId: string, addresses: string[]) => {
    await verifyCreator(launchId, creatorId);

    let addedCount = 0;
    for (const address of addresses) {
        try {
            await prisma.whitelist.create({ data: { launchId, address } });
            addedCount++;
        } catch (err: any) {
            if (err.code !== "P2002") throw err;
        }
    }

    const total = await prisma.whitelist.count({ where: { launchId } });
    return { added: addedCount, total };
};

export const getAddresses = async (launchId: string, userId: string) => {
    await verifyCreator(launchId, userId);

    const entries = await prisma.whitelist.findMany({
        where: { launchId },
        select: { address: true },
    });

    return {
        addresses: entries.map((e) => e.address),
        total: entries.length,
    };
};

export const removeAddress = async (launchId: string, creatorId: string, address: string) => {
    await verifyCreator(launchId, creatorId);

    const entry = await prisma.whitelist.findUnique({
        where: { launchId_address: { launchId, address } },
    });

    if (!entry) throw new AppError("Address not found in whitelist", 404);

    await prisma.whitelist.delete({ where: { id: entry.id } });
    return { removed: true };
};

export const isWhitelisted = async (launchId: string, walletAddress: string) => {
    const count = await prisma.whitelist.count({ where: { launchId } });
    if (count === 0) return true;

    const entry = await prisma.whitelist.findUnique({
        where: { launchId_address: { launchId, address: walletAddress } },
    });
    return !!entry;
};

const verifyCreator = async (launchId: string, userId: string) => {
    const launch = await prisma.launch.findUnique({ where: { id: launchId } });
    if (!launch) throw new AppError("Launch not found", 404);
    if (launch.creatorId !== userId) {
        throw new AppError("Forbidden: not the launch creator", 403);
    }
    return launch;
};
