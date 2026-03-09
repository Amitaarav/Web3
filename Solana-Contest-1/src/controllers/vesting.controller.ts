import { Request, Response } from "express";
import * as vestingService from "../services/vesting.service";
import { asyncHandler, AppError } from "../utils/errors";

export const getVesting = asyncHandler(async (req: Request, res: Response) => {
    const { walletAddress } = req.query;

    if (!walletAddress) {
        throw new AppError("walletAddress query parameter is required", 400);
    }

    const schedule = await vestingService.getVestingSchedule(
        req.params.id,
        walletAddress as string
    );
    res.status(200).json(schedule);
});
