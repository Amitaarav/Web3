import { Response } from "express";
import { AuthRequest } from "../middleware/auth";
import * as purchaseService from "../services/purchase.service";
import { asyncHandler } from "../utils/errors";
import { requireFields } from "../utils/validators";

export const createPurchase = asyncHandler(async (req: AuthRequest, res: Response) => {
    requireFields(req.body, ["walletAddress", "amount", "txSignature"]);

    const purchase = await purchaseService.createPurchase(
        req.params.id,
        req.user!.id,
        req.body
    );
    res.status(201).json(purchase);
});

export const listPurchases = asyncHandler(async (req: AuthRequest, res: Response) => {
    const result = await purchaseService.listPurchases(
        req.params.id,
        req.user!.id
    );
    res.status(200).json(result);
});
