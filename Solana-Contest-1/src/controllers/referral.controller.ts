import { Response } from "express";
import { AuthRequest } from "../middleware/auth";
import * as referralService from "../services/referral.service";
import { asyncHandler } from "../utils/errors";
import { requireFields } from "../utils/validators";

export const createReferral = asyncHandler(async (req: AuthRequest, res: Response) => {
    requireFields(req.body, ["code", "discountPercent", "maxUses"]);

    const referral = await referralService.createReferral(
        req.params.id,
        req.user!.id,
        req.body
    );
    res.status(201).json(referral);
});

export const listReferrals = asyncHandler(async (req: AuthRequest, res: Response) => {
    const referrals = await referralService.listReferrals(
        req.params.id,
        req.user!.id
    );
    res.status(200).json(referrals);
});
