import { Router } from "express";
import { authenticate } from "../middleware/auth";
import * as referralController from "../controllers/referral.controller";

const router = Router({ mergeParams: true });

router.post("/", authenticate as any, referralController.createReferral);
router.get("/", authenticate as any, referralController.listReferrals);

export default router;
