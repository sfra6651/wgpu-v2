# HD Top-Down Art & Animation Pipeline — Options Review

Context: `wgpu-v2`, custom Rust/wgpu ECS engine, 8-directional top-down dark fantasy ARPG.
Current pipeline: PixelLab pixel sprites, 8 idle frames + 8 dirs × 4 run frames per character, one PNG → one bind group.
Goal: clean, semi-vector, HD look. No in-house art capability. AI must do the heavy lifting.

---

## 1. The reframe that makes this decidable

You are treating this as one problem ("AI can't do directions"). It's actually two, and they have completely different solutions.

**Problem 1 — Style consistency.** Does every asset look like it came from the same game? Solvable with AI today. Train a custom style model / LoRA on ~20–30 reference images (Scenario, Layer.ai) or lock a rigid prompt + palette + lighting recipe. This is a solved problem and it's the *only* one AI image generation actually solves.

**Problem 2 — Identity & geometry consistency across rotation and time.** Is the character in frame 3 facing NE the *same object* as the character in frame 1 facing S — same shoulder pad, same buckle, same silhouette volume, same pivot? **No 2D image generator solves this and none will soon.** PixelLab appears to solve it only because at 32–64px, the information budget is so small that "close enough" reads as identical. The moment you go HD, every inconsistency AI produces becomes visible — which is exactly the wall you've hit.

So the rule is: **anything that works does so by making rotation stop being a generation problem.** There are only three ways to do that:

- **Render it** — build the thing once in 3D, rotate a camera. (Options A, F)
- **Rig it** — build the thing once in parts, rotate the parts. (Options B, C, D)
- **Avoid it** — design so that rotation is cheap or unnecessary. (Option E — combines with everything)

Everything below is one of those three.

---

## 2. The asset math (why you must pick a *machine*, not a *process*)

Current shipping set is thin: 40 frames/character. A real ARPG needs roughly:

| Animation | Frames | × 8 dirs |
|---|---|---|
| Idle | 4 | 32 |
| Run | 6 | 48 |
| Attack (×2 variants) | 6 each | 96 |
| Hit react | 2 | 16 |
| Death | 8 | 64 |
| **Total** | **32** | **~256 frames per character** |

At 256×256 RGBA that's ~67 MB uncompressed per character. Ten characters is ~670 MB.

Two conclusions fall out immediately:

1. **Any workflow with a human in the loop per frame is dead.** 256 frames × 10 characters = 2,560 generate-inspect-fix cycles. Even at 30s each that's 21 hours of pure clicking, and it will not be consistent at the end. You need a pipeline where adding an animation costs a *button press*, not a generation session.
2. **Your renderer needs atlasing before you go HD** (see §5). One-bind-group-per-PNG at 256px will fall over.

This math is the single strongest argument for Option A.

---

## 3. The options

### Option A — 3D → prerendered 2D sprites ⭐ recommended

The classic method: *Diablo I/II, Baldur's Gate, Fallout 1/2, Don't Starve, Hades (partly), most modern "HD 2D" ARPGs.*

**Pipeline:**
1. Get a mesh once per character. Either AI image-to-3D (Meshy, Tripo, Rodin, Hunyuan3D — feed it your one good AI concept image), or a bought/free base mesh kit, or a CC0 humanoid + kitbashed gear.
2. Auto-rig it — Mixamo is free, handles humanoid bipeds, and ships a library of run/idle/attack/death animations you can retarget instantly. Non-humanoids need manual rigging or a purchased rig.
3. Set up a Blender scene *once*: orthographic camera on an 8-position turntable at your game's pitch angle, flat/toon shader (Emission + colour ramp, no PBR), inverse-hull or Freestyle outline pass, transparent film.
4. Press render. Out come 8 dirs × N frames, pixel-perfect consistent, at any resolution you want.

**Why it wins here:** it *structurally cannot* produce a directional inconsistency — it's the same geometry from a different camera. It's the only option where the marginal cost of "add a dodge-roll animation" is near zero. It's also the only one that gives you **normal maps, depth, ambient-occlusion and material masks for free** as extra render passes, which means your wgpu renderer can do real per-pixel dynamic lighting on sprites — torches, spell glow, day/night. That is a bigger visual differentiator than the base art itself, and no AI tool can hand it to you.

**The clean vector look is directly achievable** — flat toon shading with a hard outline pass *is* the "clean HD vector" aesthetic. That's exactly how it's usually made.

**Costs / risks:**
- Blender learning curve. Real but bounded, and mostly one-time: you build the render rig once and it's Python-scriptable, so you can automate the whole export from a CLI. As a programmer this is the version of "art tooling" you're best equipped to attack.
- AI-generated meshes are lumpy and badly topologised. Fine for a toon-shaded 96px-on-screen character; not fine if you zoom. Expect cleanup, or use them as blockouts.
- Mixamo retargeting only handles humanoids. Goblins, orcs, undead: fine. Spiders, dragons, floating things: manual work.
- Iteration is slower than 2D — a silhouette change means going back into the 3D scene.

**Verdict:** highest ceiling, best cost curve, and the only option that survives contact with the §2 asset math. The upfront investment is 1–2 weeks of learning; it pays back at roughly the third character.

---

### Option B — 2D skeletal animation (Spine / DragonBones / Spriter)

Generate one HD turnaround per character (S / SE / E / N poses), cut into parts — head, torso, upper/lower arm, legs, weapon, cape — rig each part to a skeleton, animate once, reuse across directions.

**Pipeline:** AI concept art → segment into layers (SAM-based tools, or Photoshop/Krita by hand) → rig in Spine → export either baked sprite sheets, or run the skeleton at runtime.

**Rust runtime is viable:** [`rusty_spine`](https://github.com/jabuwu/rusty_spine) is the official C runtime transpiled to Rust, works with wasm, and is renderer-agnostic — you feed it into your own wgpu pipeline. [`spine2d`](https://github.com/Latias94/spine2d) is a newer pure-Rust attempt. You can also sidestep runtime integration entirely by baking sheets from Spine, at the cost of memory and losing procedural control.

**Pros:**
- Best-in-class polish for hand-crafted 2D. Secondary motion, cloth, IK-driven aiming, look-at, foot-planting.
- Runtime skeletons are tiny in memory — dozens of characters for the cost of a few atlases.
- Equipment swapping and recolours are basically free (swap an attachment).
- Procedural aim: rotate the weapon bone toward the cursor with no extra art.

**Cons:**
- **Rigging is genuine craft work, per character.** This is the option that most contradicts "I don't have the capability to make art."
- 8 directions still needs ~5 distinct rigs (S, SE, E, NE, N; mirror for the west side) with the same animation data retargeted — so ~5× the rigging, not 1×.
- Spine Pro (mesh deformation, which you want) is a paid one-off licence.
- Part-cutting AI art into clean layers with correct occlusion is fiddly.

**Verdict:** the best *look* if you're willing to become a rigger. You've said you aren't. Keep it as the option you graduate to for the player character specifically, once the game is worth that investment.

---

### Option C — Procedural cut-out / "floating weapon" (your idea, sharpened)

Static directional body sprite (8 per character) + separately-generated weapon, and optionally head/hands, layered on top and animated **entirely in code**: rotation, offset, squash/stretch, bob, lean, step-cadence, shadow scaling, hit-flash.

This is a real, shipped, widely-used technique. *Enter the Gungeon, Nuclear Throne, Brotato, Vampire Survivors, Realm of the Mad God* and most of the twin-stick/roguelite space use some flavour of it — body carries the pose, the arm and weapon are a separate rotating layer.

**Your engine is already 80% there.** You have `Rotation`, `RenderSize`, a model-matrix path in `model_transforms.rs`, and a `DrawEntity` list. A weapon layer is one extra draw entry with a pivot offset and its own rotation. This is the cheapest option to *implement* by a wide margin.

**Pros:**
- Minimal asset count: 8 body images + 1 weapon image per character, versus 256 frames.
- Procedural aiming falls out naturally — weapon points at the cursor in 360°, not 8 steps, which actually feels *better* than sprite-based aiming.
- Weapon variety is free and combinatorial: 20 weapons × 10 characters = 30 assets, not 200 animation sets.
- Trivially extends to hit-flash, knockback lean, dash-stretch, all in code.

**Cons:**
- **You still have to get 8 consistent HD body sprites**, and that's the exact thing AI is bad at. This option reduces the problem by ~30× but does not solve it. (Realistically: generate the 8 directions via Option A's 3D render, then animate procedurally — see the hybrid below.)
- The body itself doesn't animate. Procedural bob/lean sells stylised and cartoony well; it reads as stiff for grounded dark fantasy at HD resolution. Your world bible is "stylized realism" — that's the harder end for this technique.
- Layering order across 8 directions needs care (weapon behind the body when facing N, in front when facing S) — solvable with a per-direction sort key, just don't forget it.

**Verdict:** excellent as a *component*, weak as the *whole* answer. Which points at:

### ⭐ Option A+C hybrid — the actual recommendation

- Bodies: 3D-rendered, 8 directions, short loops only (idle 4f, run 6f). ~80 frames/character, not 256.
- Weapons, shields, spell effects, projectiles: separate flat assets, rotated and moved procedurally at runtime.
- Attacks, dashes, hit reactions, deaths: **procedural**, driven by the weapon/body transforms — a swing is the weapon bone arcing plus a body lean plus a squash, not 48 rendered frames.

This cuts the 3D workload by ~two-thirds, keeps directional consistency structurally guaranteed, gives 360° aiming instead of 8-step, and makes new weapons nearly free. It plays directly to your strengths — you'd be writing motion code instead of making art.

---

### Option D — True vector runtime (Rive / SVG)

Author or convert art to vector, animate with a state machine, render as vectors at runtime.

- **Rive** is the serious contender: vector, state machines, tiny files, and an official Rust runtime exists. Genuinely resolution-independent — the literal "clean HD vector" you described.
- **SVG → texture bake** is the lo-fi version: vectorise AI raster output (Vectorizer.AI, Illustrator Image Trace, potrace), render to textures at your target size at build time.

**Cons:** rive-rs integration into a hand-rolled wgpu renderer is not a paved road — you'd be doing engine work, possibly a lot of it. AI raster → clean vector conversion is unreliable for organic shapes (it produces hundreds of noisy paths, not clean designed ones). And a top-down 8-direction vector character is still a rigging job — Rive gives you the renderer, not the rotation problem.

**Verdict:** the aesthetic is exactly right, the path to it is the least paved. Worth 30 minutes of reading, not worth betting the project on unless you want the engine work for its own sake.

---

### Option E — Design your way out of the problem (do this regardless)

These are free and they compound with every option above:

- **Mirror the west side.** S, SE, E, NE, N are 5 unique directions; SW/W/NW are horizontal flips. That's a 37% asset cut for one line of shader/UV code. Caveat: asymmetric details (a sword always in the right hand, a scar, a shoulder crest) will flip. In practice almost nobody notices; if you care, design characters symmetrically.
- **Consider 4 directions instead of 8.** Halves everything again. Many well-regarded top-down games ship 4.
- **Consider steeper overhead camera.** The closer to true bird's-eye, the more a character approximates rotational symmetry — at the extreme you render *one* sprite and rotate it programmatically, and the directional problem vanishes entirely. Cost: you lose faces and character personality, which is a real loss for a story-driven dark fantasy RPG. Probably not for you, but it's the nuclear option and it does work.
- **Design silhouettes that hide what AI is worst at.** Hoods, helmets, cloaks, masks, pauldrons, robes. Faces and hands are where AI consistency collapses; your world bible ("hooded conduits, decaying institutions, ancient orders") gives you total licence to cover both. This is the highest-leverage art-direction decision available to you and it costs nothing.
- **Lock a style bible.** ~20 reference images, fixed palette, fixed light direction (top-left is conventional), fixed outline weight, fixed camera pitch. Feed it to a custom-trained style model (Scenario, Layer.ai). Even with a 3D pipeline this governs your concept art and all your 2D VFX/UI.

---

### Option F — Just render 3D characters at runtime

You're already writing raw wgpu. Toon-shaded low-poly characters rendered into an orthographic top-down view, with 2D tiles and sprites for the world. Rotation becomes a matrix multiply — the problem disappears completely, along with all the sprite memory concerns.

**Pros:** infinite directions, free smooth turning, real dynamic lighting, no sprite atlases, equipment swapping is mesh swapping.
**Cons:** glTF loading, skinned-mesh skeletal animation, animation blending, a whole second render pipeline. That's a serious engine detour — realistically a month-plus before it looks as good as sprites do today. It also drifts from the 2D aesthetic you actually want.

**Verdict:** note that Option A is the same 3D asset work *without* the engine work. If you find yourself doing Option A and enjoying the 3D side, this becomes a live option later — the assets carry over. Don't start here.

---

### Option G — Keep fighting the 2D generators

Reference-conditioned editing models (Flux Kontext, nano-banana-class, Seedream), ControlNet pose transfer, and purpose-built sprite tools ([SpriteFlow](https://spriteflow.io/direction-sprite-generator) claims 8-direction turnaround from one image; [Scenario](https://www.scenario.com/) has 360-spin and sprite-sheet features on custom-trained models).

**Honest assessment:** these are meaningfully better than 2024-era tools and worth an afternoon of testing — SpriteFlow in particular is aimed exactly at your problem. But they're all attacking Problem 2 with a Problem 1 tool. Expect them to be *usable* for a hero character where you'll hand-fix artefacts, and *not* usable as a production machine for 256 frames × 10 characters. Test them, but don't architect around them.

---

## 4. Comparison

| | Directional consistency | Art skill needed | Setup cost | Cost per new anim | Cost per new character | Fits your engine |
|---|---|---|---|---|---|---|
| **A. 3D prerender** | Guaranteed | Low–med (Blender) | High (1–2 wks) | ~Zero | Medium | Needs atlasing |
| **B. Spine rig** | Guaranteed | **High** | Med | Low | **High** | Runtime integration |
| **C. Procedural cut-out** | Doesn't solve it | Low | **Very low** | ~Zero | Low | **Already 80% built** |
| **A+C hybrid** ⭐ | Guaranteed | Low–med | High | Low | Low–med | Needs atlasing |
| **D. Rive/vector** | Doesn't solve it | Med–high | High (engine) | Low | High | Significant work |
| **E. Design cuts** | n/a — multiplier | None | ~Zero | n/a | n/a | Trivial |
| **F. Runtime 3D** | Guaranteed | Low–med | **Very high** | ~Zero | Medium | New pipeline |
| **G. 2D AI gen** | **No** | Low | Low | High | High | No change |

---

## 5. What changes in `wgpu-v2` regardless of which you pick

Going HD breaks assumptions in the current renderer. Worth knowing before you commit:

1. **Atlas or texture-array your sprites.** `character_sprite_set.rs` currently does one `wgpu::BindGroup` per PNG, with paths formatted at load time. At 256×256 that's 256 bind groups and ~67 MB per character, plus a bind-group rebind per draw. Move to a texture atlas (or a `texture_2d_array`) with UV offsets in an instance buffer — then all characters draw in one or two batches. Do this *before* the art migration, not after; it's much easier on the small pixel assets you have now.
2. **Block-compress your textures.** BC7 (or ASTC on Apple Silicon) is ~4:1 with no visible loss on this kind of art. 670 MB → ~170 MB. `wgpu` supports these natively; you'd add an offline bake step.
3. **Mirroring is a UV sign flip**, not extra textures. `Facing` already enumerates 8; map SW/W/NW to their mirrors and drop 3/8 of your memory.
4. **Extend `Facing` resolution or go continuous.** With a procedural weapon layer you'll want the aim direction as a float angle, not an 8-way enum. Consider keeping `Facing` for body sprite selection and adding a separate `AimAngle(f32)` for the weapon layer.
5. **Layered draws need a sort key.** `DrawEntity` currently has one texture per entity. Body + weapon + shadow + VFX means multiple draws per entity with a stable order that varies by facing. A `(layer, y_position, sub_layer)` sort before writing transforms is the standard fix — you'll want y-sorting anyway for a top-down game.
6. **Ambition option: normal-mapped sprites.** If you go with Option A, a second render pass gives you a normal map per frame for free. Sample it in `texture.wgsl` and you get real dynamic lighting on 2D sprites. This is the thing that will make your game look expensive, and it's only available on the 3D-render path.
7. **`Action` enum is currently Idle/Run only.** Whatever you choose, plan the state machine (attack, hit, death, dash, cast) before generating 256 frames against the wrong list.

---

## 6. Recommended next step: a two-day spike

Don't decide from this document — decide from a screenshot. Do both of these against **one goblin**, then look at them side by side in the actual game:

**Spike 1 — Option A (1–1.5 days).**
AI concept image of a goblin → Meshy/Tripo → mesh → Mixamo auto-rig + a stock run cycle → Blender scene with orthographic camera at your game pitch, flat toon shader, inverse-hull outline → render 5 directions × 6 frames → drop into `src/assets/goblin/`. Question to answer: *does the toon-shaded render look like the clean HD style I want, and how painful was the mesh cleanup?*

**Spike 2 — Option C (0.5 day).**
Take one good AI-generated static goblin body + a separate axe sprite. Add a second `DrawEntity` for the weapon with a pivot offset and its own rotation, driven by aim direction. Add a code-driven run bob (sine on Y + slight rotation) and a squash on footfall. Question to answer: *does procedural motion feel alive enough to carry most of the animation budget?*

Then combine what worked. My expectation is that Spike 1 answers "yes, and it's more approachable than I feared" and Spike 2 answers "yes, and it feels better than sprite-stepped aiming" — which lands you on the A+C hybrid.

**Also update §2 of the world bible** — it currently mandates pixel art as the medium. The rest of that section (stylized realism, clean graphic design, strong readability, vibrant not desaturated) survives the change intact and is exactly right for a toon-shaded 3D render pipeline. Only the word "pixel" needs to go.

---

## Sources

- [rusty_spine — Spine runtime for Rust](https://github.com/jabuwu/rusty_spine)
- [spine2d — pure Rust Spine runtime (experimental)](https://github.com/Latias94/spine2d)
- [Blender Spritesheet Renderer](https://github.com/chrishayesmu/Blender-Spritesheet-Renderer)
- [Sprite Sheet Generator — Blender Extensions](https://extensions.blender.org/add-ons/sprite-sheet-generator/)
- [8 Directions Render Plugin for Blender](https://auteddy.gumroad.com/l/8d_blender_plugin)
- [Tripo AI — image to 3D](https://www.tripo3d.ai/features/image-to-3d-model)
- [Meshy — AI tools for 3D game assets](https://www.meshy.ai/blog/best-ai-tools-for-3d-game-assets)
- [SpriteFlow — 8-direction sprite generator](https://spriteflow.io/direction-sprite-generator)
- [Scenario — custom model game asset generation](https://help.scenario.com/articles/1129045411-frequently-asked-questions-faq)
- [Best AI 2D game asset generators, 2026 comparison](https://www.summerengine.com/blog/ai-2d-game-asset-generator)
