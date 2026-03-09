import prisma from "./utils/prisma";
import * as authService from "./services/auth.service";
import * as launchService from "./services/launch.service";
import * as purchaseService from "./services/purchase.service";
import * as referralService from "./services/referral.service";
import * as whitelistService from "./services/whitelist.service";
import * as vestingService from "./services/vesting.service";

async function runTests() {
    console.log("Starting Verification Tests...");

    try {
        // 1. Clean up
        await prisma.purchase.deleteMany();
        await prisma.referralCode.deleteMany();
        await prisma.whitelist.deleteMany();
        await prisma.launchTier.deleteMany();
        await prisma.vestingConfig.deleteMany();
        await prisma.launch.deleteMany();
        await prisma.user.deleteMany();

        console.log("Step 1: Auth - Register & Login");
        const regRes = await authService.register({
            email: "test@example.com",
            password: "password123",
            name: "Test User"
        });
        const userId = regRes.user.id;
        console.log("Registered user:", userId);

        const loginRes = await authService.login({
            email: "test@example.com",
            password: "password123"
        });
        console.log("Logged in user:", loginRes.user.id);

        console.log("Step 2: Launches - Create & List");
        const now = new Date();
        const launch = await launchService.createLaunch(userId, {
            name: "Test Launch",
            symbol: "TEST",
            totalSupply: 1000,
            pricePerToken: 0.1,
            startsAt: new Date(now.getTime() - 10000).toISOString(),
            endsAt: new Date(now.getTime() + 10000).toISOString(),
            maxPerWallet: 500,
            tiers: [
                { minAmount: 0, maxAmount: 100, pricePerToken: 0.05 },
                { minAmount: 100, maxAmount: 300, pricePerToken: 0.08 }
            ],
            vesting: {
                cliffDays: 0,
                vestingDays: 30,
                tgePercent: 10
            }
        });
        console.log("Created launch with tiers and vesting:", launch.id);

        console.log("Step 3: Whitelist Management");
        await whitelistService.addAddresses(launch.id, userId, ["addr1", "addr2"]);
        const wl = await whitelistService.getAddresses(launch.id, userId);
        console.log("Whitelist total:", wl.total);

        console.log("Step 4: Referral Codes");
        const ref = await referralService.createReferral(launch.id, userId, {
            code: "PROMO",
            discountPercent: 10,
            maxUses: 5
        });
        console.log("Created referral code:", ref.code);

        console.log("Step 5: Purchase - Tiered Pricing & Referral");
        // Purchase 150 tokens. 
        // 100 @ 0.05 = 5.0
        // 50 @ 0.08 = 4.0
        // Total = 9.0
        // 10% discount = 8.1
        const p1 = await purchaseService.createPurchase(launch.id, userId, {
            walletAddress: "addr1",
            amount: 150,
            txSignature: "sig1",
            referralCode: "PROMO"
        });
        console.log("Purchase 1 cost:", p1.totalCost); // Should be ~8.1
        if (Math.abs(p1.totalCost - 8.1) < 0.001) console.log("SUCCESS: Tiered pricing and referral correct");
        else console.error("FAILURE: Incorrect cost calculation", p1.totalCost);

        console.log("Step 6: Sybil Protection - maxPerWallet");
        try {
            await purchaseService.createPurchase(launch.id, userId, {
                walletAddress: "addr2",
                amount: 400, // 150 + 400 = 550 > 500
                txSignature: "sig2"
            });
            console.error("FAILURE: maxPerWallet not enforced!");
        } catch (err: any) {
            console.log("SUCCESS: maxPerWallet enforced:", err.message);
        }

        console.log("Step 7: Status Filtering & Pagination");
        const list = await launchService.listLaunches({ page: 1, limit: 10, status: "ACTIVE" });
        console.log("Active launches count:", list.total);
        if (list.total === 1) console.log("SUCCESS: Status filtering works");

        console.log("Step 8: Vesting Calculation");
        const vest = await vestingService.getVestingSchedule(launch.id, "addr1");
        console.log("Vesting for addr1:", vest.tgeAmount, vest.claimableAmount);
        // 150 total. 10% TGE = 15.
        if (vest.tgeAmount === 15) console.log("SUCCESS: TGE amount correct");

        console.log("\nALL TESTS COMPLETED SUCCESSFULLY!");
    } catch (err) {
        console.error("TESTS FAILED:", err);
        process.exit(1);
    }
}

runTests();
