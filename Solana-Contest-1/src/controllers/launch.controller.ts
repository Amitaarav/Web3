import { Request, Response } from "express";
import { AuthRequest } from "../middleware/auth";
import * as launchService from "../services/launch.service";
import { asyncHandler, AppError } from "../utils/errors";
import { requireFields } from "../utils/validators";

export const createLaunch = asyncHandler(async (req: AuthRequest, res: Response) => {
    if (!req.body) throw new AppError("Missing request body", 400);
    requireFields(req.body, [
        "name", "symbol", "totalSupply", "pricePerToken",
        "startsAt", "endsAt", "maxPerWallet",
    ]);

    const launch = await launchService.createLaunch(req.user!.id, req.body);
    res.status(201).json(launch);
});

export const listLaunches = asyncHandler(async (req: any, res: Response) => {
    const page = parseInt(req.query.page as string) || 1;
    const limit = parseInt(req.query.limit as string) || 10;
    const status = req.query.status as string;

    const result = await launchService.listLaunches({ page, limit, status });
    res.status(200).json(result);
});

export const getLaunch = asyncHandler(async (req: any, res: Response) => {
    const launch = await launchService.getLaunchById(req.params.id);
    res.status(200).json(launch);
});

export const updateLaunch = asyncHandler(async (req: AuthRequest, res: Response) => {
    const launch = await launchService.updateLaunch(
        req.params.id,
        req.user!.id,
        req.body
    );
    res.status(200).json(launch);
});
