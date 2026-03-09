import { Router } from "express";
import { authenticate } from "../middleware/auth";
import * as launchController from "../controllers/launch.controller";

import whitelistRoutes from "./whitelist";
import referralRoutes from "./referrals";
import purchaseRoutes from "./purchases";
import vestingRoutes from "./vesting";

const router = Router();

router.post("/", authenticate as any, launchController.createLaunch);
router.get("/", launchController.listLaunches);
router.get("/:id", launchController.getLaunch);
router.put("/:id", authenticate as any, launchController.updateLaunch);

router.use("/:id/whitelist", whitelistRoutes);
router.use("/:id/referrals", referralRoutes);
router.use("/:id/purchase", purchaseRoutes);
router.use("/:id/purchases", purchaseRoutes);
router.use("/:id/vesting", vestingRoutes);

export default router;
