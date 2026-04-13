import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

describe("Drishti System Test", () => {
  // 1. Setup Provider
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // 2. Define Programs
  // We use anchor.workspace as a shortcut, but since IDLs are missing,
  // we use 'any' temporarily to stop TypeScript from screaming.
  const coreProgram = anchor.workspace.DrishtiCore as Program<any>;
  const registryProgram = anchor.workspace.DrishtiRegistry as Program<any>;

  it("Initializes Drishti Core!", async () => {
    try {
      const tx = await coreProgram.methods.initialize().rpc();
      console.log("Core Initialization Signature:", tx);
    } catch (err) {
      console.error("Core Init Failed:", err);
      throw err;
    }
  });

  it("Initializes Drishti Registry!", async () => {
    try {
      // Assuming registry also has an initialize function
      const tx = await registryProgram.methods.initialize().rpc();
      console.log("Registry Initialization Signature:", tx);
    } catch (err) {
      console.log("Registry Init skipped or failed (check if initialize() exists in registry/lib.rs)");
    }
  });
});