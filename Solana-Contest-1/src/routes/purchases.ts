import { Router } from "express";
import { authenticate } from "../middleware/auth";
import * as purchaseController from "../controllers/purchase.controller";

const router = Router({ mergeParams: true });

router.post("/", authenticate as any, purchaseController.createPurchase);
router.get("/", authenticate as any, purchaseController.listPurchases);

export default router;
