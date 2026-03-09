import { Router } from "express";
import * as vestingController from "../controllers/vesting.controller";

const router = Router({ mergeParams: true });

router.get("/", vestingController.getVesting);

export default router;
