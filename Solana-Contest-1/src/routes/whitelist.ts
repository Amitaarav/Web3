import { Router } from "express";
import { authenticate } from "../middleware/auth";
import * as whitelistController from "../controllers/whitelist.controller";

const router = Router({ mergeParams: true });

router.post("/", authenticate as any, whitelistController.addAddresses);
router.get("/", authenticate as any, whitelistController.getAddresses);
router.delete("/:address", authenticate as any, whitelistController.removeAddress);

export default router;
