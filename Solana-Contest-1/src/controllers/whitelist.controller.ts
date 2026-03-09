import { Response } from "express";
import { AuthRequest } from "../middleware/auth";
import * as whitelistService from "../services/whitelist.service";
import { asyncHandler, AppError } from "../utils/errors";

export const addAddresses = asyncHandler(async (req: AuthRequest, res: Response) => {
    const { addresses } = req.body;
    if (!addresses || !Array.isArray(addresses) || addresses.length === 0) {
        throw new AppError("addresses must be a non-empty array", 400);
    }

    const result = await whitelistService.addAddresses(
        req.params.id,
        req.user!.id,
        addresses
    );
    res.status(200).json(result);
});

export const getAddresses = asyncHandler(async (req: AuthRequest, res: Response) => {
    const result = await whitelistService.getAddresses(
        req.params.id,
        req.user!.id
    );
    res.status(200).json(result);
});

export const removeAddress = asyncHandler(async (req: AuthRequest, res: Response) => {
    const result = await whitelistService.removeAddress(
        req.params.id,
        req.user!.id,
        req.params.address
    );
    res.status(200).json(result);
});
