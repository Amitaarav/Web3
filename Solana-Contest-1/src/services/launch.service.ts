import prisma from "../utils/prisma";
import { AppError } from "../utils/errors";
import { Launch, LaunchTier, VestingConfig } from "@prisma/client";

export const computeStatus = (launch: Launch, totalPurchased: number) => {
    const now = new Date();
    if (totalPurchased >= launch.totalSupply) return "SOLD_OUT";
    if (now < new Date(launch.startsAt)) return "UPCOMING";
    if (now > new Date(launch.endsAt)) return "ENDED";
    return "ACTIVE";
};

export const enrichLaunch = async (launch: any) => {
    const agg = await prisma.purchase.aggregate({
        where: { launchId: launch.id },
        _sum: { amount: true },
    });
    const totalPurchased = agg._sum.amount || 0;
    return { ...launch, status: computeStatus(launch, totalPurchased) };
};

const launchIncludes = {
    tiers: true,
    vesting: true,
};

export const createLaunch = async (creatorId: string, data: any) => {
    const {
        name, symbol, totalSupply, pricePerToken,
        startsAt, endsAt, maxPerWallet, description,
        tiers, vesting,
    } = data;

    const launch = await prisma.launch.create({
        data: {
            creatorId,
            name,
            symbol,
            totalSupply,
            pricePerToken,
            startsAt: new Date(startsAt),
            endsAt: new Date(endsAt),
            maxPerWallet,
            description: description || null,
            ...(tiers && tiers.length > 0 && {
                tiers: {
                    create: tiers.map((t: any) => ({
                        minAmount: t.minAmount,
                        maxAmount: t.maxAmount,
                        pricePerToken: t.pricePerToken,
                    })),
                },
            }),
            ...(vesting && {
                vesting: {
                    create: {
                        cliffDays: vesting.cliffDays,
                        vestingDays: vesting.vestingDays,
                        tgePercent: vesting.tgePercent,
                    },
                },
            }),
        },
        include: launchIncludes,
    });

    return enrichLaunch(launch);
};

export const enrichLaunches = async (launches: any[]) => {
    if (launches.length === 0) return [];

    const launchIds = launches.map((l) => l.id);
    const aggregates = await prisma.purchase.groupBy({
        by: ["launchId"],
        where: { launchId: { in: launchIds } },
        _sum: { amount: true },
    });

    const sumsMap = new Map(aggregates.map((a) => [a.launchId, a._sum.amount || 0]));

    return launches.map((l) => ({
        ...l,
        status: computeStatus(l, sumsMap.get(l.id) || 0),
    }));
};

export const listLaunches = async ({
    page = 1,
    limit = 10,
    status,
}: {
    page?: number;
    limit?: number;
    status?: string;
}) => {
    const skip = (page - 1) * limit;
    const now = new Date();

    let where: any = {};
    if (status) {
        const s = status.toUpperCase();
        if (s === "UPCOMING") {
            where.startsAt = { gt: now };
        } else if (s === "ENDED") {
            where.endsAt = { lt: now };
        } else if (s === "ACTIVE" || s === "SOLD_OUT") {
            // SOLD_OUT is typically either ACTIVE or ENDED, but we'll fetch both to be safe
            // and filter accurately in JS
            where.startsAt = { lte: now };
        }
    }

    if (status) {
        // When status is provided, we fetch all candidates and filter/paginate in JS
        // to ensure correctness since status is computed
        const allCandidates = await prisma.launch.findMany({
            where,
            orderBy: { createdAt: "desc" },
            include: launchIncludes,
        });

        const enriched = await enrichLaunches(allCandidates);
        const filtered = enriched.filter((l) => l.status === status.toUpperCase());

        return {
            launches: filtered.slice(skip, skip + limit),
            total: filtered.length,
            page,
            limit,
        };
    } else {
        // Standard pagination when no status filter
        const [launches, total] = await Promise.all([
            prisma.launch.findMany({
                skip,
                take: limit,
                orderBy: { createdAt: "desc" },
                include: launchIncludes,
            }),
            prisma.launch.count(),
        ]);

        const enriched = await enrichLaunches(launches);

        return {
            launches: enriched,
            total,
            page,
            limit,
        };
    }
};

export const getLaunchById = async (id: string) => {
    const launch = await prisma.launch.findUnique({
        where: { id },
        include: launchIncludes,
    });

    if (!launch) throw new AppError("Launch not found", 404);
    return enrichLaunch(launch);
};

export const updateLaunch = async (id: string, userId: string, data: any) => {
    const launch = await prisma.launch.findUnique({ where: { id } });

    if (!launch) throw new AppError("Launch not found", 404);
    if (launch.creatorId !== userId) {
        throw new AppError("Forbidden: not the launch creator", 403);
    }

    const { tiers, vesting, id: _id, creatorId: _cid, createdAt: _ca, updatedAt: _ua, ...updateData } = data;

    if (updateData.startsAt) updateData.startsAt = new Date(updateData.startsAt);
    if (updateData.endsAt) updateData.endsAt = new Date(updateData.endsAt);

    const operations: any[] = [];

    // Handle tiers update
    if (tiers !== undefined) {
        operations.push(prisma.launchTier.deleteMany({ where: { launchId: id } }));
        if (Array.isArray(tiers) && tiers.length > 0) {
            operations.push(
                ...tiers.map((t: any) =>
                    prisma.launchTier.create({
                        data: {
                            launchId: id,
                            minAmount: t.minAmount,
                            maxAmount: t.maxAmount,
                            pricePerToken: t.pricePerToken,
                        },
                    })
                )
            );
        }
    }

    // Handle vesting update
    if (vesting !== undefined) {
        operations.push(prisma.vestingConfig.deleteMany({ where: { launchId: id } }));
        if (vesting && typeof vesting === "object") {
            operations.push(
                prisma.vestingConfig.create({
                    data: {
                        launchId: id,
                        cliffDays: vesting.cliffDays,
                        vestingDays: vesting.vestingDays,
                        tgePercent: vesting.tgePercent,
                    },
                })
            );
        }
    }

    // Update launch fields + related data in transaction
    operations.push(
        prisma.launch.update({
            where: { id },
            data: updateData,
            include: launchIncludes,
        })
    );

    const results = await prisma.$transaction(operations);

    // Re-fetch to get consistent data with updated relations
    const updated = await prisma.launch.findUnique({
        where: { id },
        include: launchIncludes
    });

    return enrichLaunch(updated);
};
