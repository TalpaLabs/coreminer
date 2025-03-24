# Changelog

## [0.5.0](https://github.com/TalpaLabs/coreminer/compare/v0.4.0...v0.5.0)

### ⛰️ Features

- Add logfiles and quiet mode - ([7df3fc7](https://github.com/TalpaLabs/coreminer/commit/7df3fc70841e993ce502350e5e01337bc3862307))

### 🐛 Bug Fixes

- [**breaking**] Last_signal was deleted after processing any status - ([e1e8aa0](https://github.com/TalpaLabs/coreminer/commit/e1e8aa0fdaa4b790f7cca4047f3b21e170420fe6))

### 📚 Documentation

- Add TalpaLabs to license - ([7ad5459](https://github.com/TalpaLabs/coreminer/commit/7ad54594795b844cadacf00dd00f67aea4ccffdd))
- Rebrand debugger-bs to TalpaLabs - ([0e6d0ad](https://github.com/TalpaLabs/coreminer/commit/0e6d0ad70eae6ac015d5e2f090fdb7463bb5b2bd))
- Fix docs and examples for last_signal fix - ([c419160](https://github.com/TalpaLabs/coreminer/commit/c41916072af59e1d447e8e0e287c0a08b4e14e8f))

### ⚙️ Miscellaneous Tasks

- Automatic Rust CI changes - ([0d3f816](https://github.com/TalpaLabs/coreminer/commit/0d3f8168fa92dd0eda118cbbc18cdd6250705f3a))


## [0.4.0](https://github.com/TalpaLabs/coreminer/compare/v0.3.0...v0.4.0)

### ⛰️ Features

- _(baseui)_ Enable or disable plugins - ([8c98db2](https://github.com/TalpaLabs/coreminer/commit/8c98db2191468081f6915e087ae679081ce94a06))
- _(cmserve)_ Add example feedback and statuses for plugins - ([df8216d](https://github.com/TalpaLabs/coreminer/commit/df8216d717845366394eaf358dca0466c8088349))
- _(plugin)_ Add sigtrap-guard plugin which detects self inserted int3 #44 - ([51a1734](https://github.com/TalpaLabs/coreminer/commit/51a17341ad3df3658776a9be1a1821d5df7f8e79))
- Add plugin list - ([ecaecc0](https://github.com/TalpaLabs/coreminer/commit/ecaecc00697217c67beb86bf43f92e7a3474d396))
- Plugin id are serialized in a transparent way - ([70e484b](https://github.com/TalpaLabs/coreminer/commit/70e484bde02d9983073cdec12ed803cc00ddf8ea))
- Enable or disable plugins in coreminer - ([7aef39d](https://github.com/TalpaLabs/coreminer/commit/7aef39d1046aa48f0d1db0227f93942b3182e48f))

### 🐛 Bug Fixes

- _(plugin)_ Breakpoint of sigtrap guard had not been stored - ([f7a50bd](https://github.com/TalpaLabs/coreminer/commit/f7a50bd7593944d20b9b8f927f33f7288f73254a))
- Arguments were not passed correctly and PATH was not used - ([183136e](https://github.com/TalpaLabs/coreminer/commit/183136ef71d2cc7282f78d415fc31b15a455e639))
- Debuggees did not properly receive their arguments - ([f5efe2b](https://github.com/TalpaLabs/coreminer/commit/f5efe2b5801507dd9fe95c2daf4b970d85c2cbf6))
- Plugin mutex was not unlocked - ([27415de](https://github.com/TalpaLabs/coreminer/commit/27415de42fce45ade1815130e032314087a5759a))

### 🚜 Refactor

- _(plugin)_ Clean up sigtrap-guard - ([75e83dd](https://github.com/TalpaLabs/coreminer/commit/75e83ddeee6150c6a2fd128d0b993ae287824035))
- [**breaking**] Serialize the cstrtings of the run status in a more human readable way - ([63ee881](https://github.com/TalpaLabs/coreminer/commit/63ee881b712e002da581b666b6dcd64c962d72f5))
- [**breaking**] A Word is now a usize to improve compatability - ([9a11434](https://github.com/TalpaLabs/coreminer/commit/9a1143461f7c58747d03097516e24b6abbe657d9))

### 📚 Documentation

- _(plugin)_ Update steckrs (doc update) and basic doc for sigtrap-guard - ([1c71602](https://github.com/TalpaLabs/coreminer/commit/1c7160245afca1c1c7061e3e314e0982d75b3535))
- Adjust readme for v0.4.0 - ([13b5941](https://github.com/TalpaLabs/coreminer/commit/13b5941dcc30f3d6f6a5b36a7ac04eef254f46c2))
- Typo in readme - ([847191b](https://github.com/TalpaLabs/coreminer/commit/847191bab928523132ae6ab8981af41131137e4d))
- Document list_plugins - ([f2f2d4d](https://github.com/TalpaLabs/coreminer/commit/f2f2d4de55a2b7c95e50fbba8c8fd0bb56a4bb2a))
- Document plugin toggle things - ([87f32c0](https://github.com/TalpaLabs/coreminer/commit/87f32c0910e50015fd990da8a054c0aa6612b360))
- Precisely disable a few more pedantic warnings - ([be052ce](https://github.com/TalpaLabs/coreminer/commit/be052cec86fcf39c51f303c0e5e63b3aed250b2c))
- Disable a warning for ser_pid and justify - ([af09b0e](https://github.com/TalpaLabs/coreminer/commit/af09b0e1e1399d353a35f3105c5a9ea71dc3d23b))
- Document the new methods in debugger - ([16bbede](https://github.com/TalpaLabs/coreminer/commit/16bbede9c62b5ba1571536dce06269bea5e651f0))
- Document sigtrap guard plugin/module - ([f579c70](https://github.com/TalpaLabs/coreminer/commit/f579c70613b500d3ade3b099ab26f6d661993301))
- Fix examples in doc for updated hook_feedback_loop - ([e1de594](https://github.com/TalpaLabs/coreminer/commit/e1de5942c8fc6352dfdb260fe429054c8ca52f1f))

### ⚙️ Miscellaneous Tasks

- _(plugin)_ Rename sigtrap guard - ([9fda062](https://github.com/TalpaLabs/coreminer/commit/9fda06215abe8a34e6a440b037f3d4811296ef09))
- Add a trace level log for the json interface - ([bcae064](https://github.com/TalpaLabs/coreminer/commit/bcae064b5d0bc2e0d09babc485290e4fa2ec2732))
- Clean up warnings for compiling without default features - ([b06693d](https://github.com/TalpaLabs/coreminer/commit/b06693dd020cb9a3dcf6c0a838419eaa8b2dee4f))
- Add required feature for hello world plugin example - ([106227a](https://github.com/TalpaLabs/coreminer/commit/106227ada57e3527a3096e8c54ab30dfd0d69937))
- Make it compile without plugins - ([11a7d0f](https://github.com/TalpaLabs/coreminer/commit/11a7d0fcf14440800dd1638b2ac7bb375f4b9efa))
- Make hello_world an example plugin - ([8f5bc4f](https://github.com/TalpaLabs/coreminer/commit/8f5bc4fc3098ea4a088946316c12045df3a8b4fe))

## [0.3.0](https://github.com/TalpaLabs/coreminer/compare/v0.2.3...v0.3.0)

### ⛰️ Features

- _(plugins)_ Implement feedback loop for hooks and enhance extension point mechanism - ([e40522e](https://github.com/TalpaLabs/coreminer/commit/e40522e10b01a531009f4610065f39da013814ab))
- _(plugins)_ Add EOnSigTrap - ([31efd7f](https://github.com/TalpaLabs/coreminer/commit/31efd7f8059b2a03592eb001c6f230fcbac59d32))
- _(plugins)_ Add plugin manager and extension point PreSignalHandler - ([8a722ad](https://github.com/TalpaLabs/coreminer/commit/8a722ad65ea5bdfa7643736c081312d8729ab602))
- Cmserve and cm loglevel trace only when compiled in debug mode - ([b5167a5](https://github.com/TalpaLabs/coreminer/commit/b5167a5b8d9ffe0398c13564ef33895a566bd1a3))
- Add hello world plugin, printing something on received signal - ([ccc51dd](https://github.com/TalpaLabs/coreminer/commit/ccc51dd6048486f045fe738d750d89346b9e22b7))

### 🐛 Bug Fixes

- For_hooks had hardcoded extension point - ([45bd2c8](https://github.com/TalpaLabs/coreminer/commit/45bd2c832c817044a8e0d213e301e2815714f249))
- Set last signal on continuing/stepping #43 - ([0f46bc3](https://github.com/TalpaLabs/coreminer/commit/0f46bc3ba576a7bcca27c094a638ec6104427c7a))
- Wait_signal stops on SIGTERM now - ([0043eba](https://github.com/TalpaLabs/coreminer/commit/0043eba3a42d2a4bdf1b991715ac78ba73a03a92))
- Plugin feedback loop was infinite - ([e735d8a](https://github.com/TalpaLabs/coreminer/commit/e735d8a5a4d8069ca8b173a41268e35f3a4aac88))
- Disable warning for private interfaces regarding internal feedback variant - ([3a29e97](https://github.com/TalpaLabs/coreminer/commit/3a29e97e65c50ac10cb5fd15c8ab4a93607376dc))
- Extension_point had been renamed - ([dd01fc3](https://github.com/TalpaLabs/coreminer/commit/dd01fc3d9c3dae2f4b0d296ca6e7fc900c5fb47d))

### 🚜 Refactor

- Remove unused method and briefly documnet Debugger::plugins - ([bc40b8e](https://github.com/TalpaLabs/coreminer/commit/bc40b8e6ad98d7af7937ab7440e74336495e3dc6))
- Move status from ui module to feedback module - ([676670b](https://github.com/TalpaLabs/coreminer/commit/676670ba57c7920e4adb0a4b6182bb600b4264d6))
- Setup default plugins and move for_hooks macro to plugins module - ([284b76b](https://github.com/TalpaLabs/coreminer/commit/284b76bf694889dfb81dc1df9da9e5899216946b))
- Add plugin feature - ([ce28786](https://github.com/TalpaLabs/coreminer/commit/ce287866ad12ee0cacdf9418929f5417a0867a0c))
- Move extension_points to plugins submodule - ([daa039e](https://github.com/TalpaLabs/coreminer/commit/daa039eef7150af187985f2936a0a6f450f21f26))
- Simplify serialization of DebuggerError - ([7d8fa38](https://github.com/TalpaLabs/coreminer/commit/7d8fa383c2b3142fd28dad24f6b2fa35c216dd82))
- Move actual processing of status to new function process_status - ([5f0a8ac](https://github.com/TalpaLabs/coreminer/commit/5f0a8aca38ede41af570a810b3a827c537b4ed97))

### 📚 Documentation

- Fix doc examples for addition of signal sending - ([695d866](https://github.com/TalpaLabs/coreminer/commit/695d866d2ce8fbba3b65cbf359c97d817f01281b))
- Fix and simplify plugin mod example - ([f72ff7e](https://github.com/TalpaLabs/coreminer/commit/f72ff7e275f169614fec29b9625a6c87f75d1d5c))
- Default plugin is HelloWorld, disabled by default - ([d3f4bf9](https://github.com/TalpaLabs/coreminer/commit/d3f4bf9ff86fd6c44f7c569c0e5bb3e64167449b))
- Add note to hello world module (it is an example plugin) - ([79df792](https://github.com/TalpaLabs/coreminer/commit/79df79205f5c882a2727982669c53119b697674b))
- Document extension_points module - ([26b13fe](https://github.com/TalpaLabs/coreminer/commit/26b13fea3bb0758546da69d308df3fbb28f21b25))
- Document plugins/mod.rs - ([992ce76](https://github.com/TalpaLabs/coreminer/commit/992ce766c8b0387f0f6e85e0bba50dcdafcc4cf4))
- Document hook_feedback_loop, make Debugger::plugins pub, fix macro for_hooks - ([224ce34](https://github.com/TalpaLabs/coreminer/commit/224ce34752499f16a8cd43a2feb1e07b41c64f72))
- Update rustdoc parts - ([b2f51a6](https://github.com/TalpaLabs/coreminer/commit/b2f51a615d2cc1e0dbe29fd01bb24c68db5e6dca))
- Add documentation to Addr::NULL - ([0ff685c](https://github.com/TalpaLabs/coreminer/commit/0ff685cdf20d8e04c1e6214d6db806c5f97a6426))

### ⚙️ Miscellaneous Tasks

- Use helloworld plugin by default (disabled) - ([2408ac8](https://github.com/TalpaLabs/coreminer/commit/2408ac8ca940a603b066e253e4b43457f1fefdf3))
- Have cargo ci only run on PR or in master branch - ([0a9e602](https://github.com/TalpaLabs/coreminer/commit/0a9e602e2293e86f9824ba91d5b53ceeabbd531e))
- Add a few status variants to the cmserve example output - ([25cf125](https://github.com/TalpaLabs/coreminer/commit/25cf1252c0a5f87d96451a93efd965c2bcc75bd5))
- Update steckrs to v0.3.\* - ([df555a4](https://github.com/TalpaLabs/coreminer/commit/df555a4b65aba4c76a33d6fdad27445d57e6ba24))
- Add signals example - ([00d2392](https://github.com/TalpaLabs/coreminer/commit/00d239247e87476149927b79ec532071de1c87af))
- Add sleeper example - ([9ccf36b](https://github.com/TalpaLabs/coreminer/commit/9ccf36bfe3db4f4e780af8d11a8fc60afee576d3))
- Add hello world plugin - ([ef1b1ca](https://github.com/TalpaLabs/coreminer/commit/ef1b1cabe5eb0d6d4af973b9ded7988b575ae0b1))
- Add comments and disable missing docs for extension_points module for now - ([08d5b28](https://github.com/TalpaLabs/coreminer/commit/08d5b28d78f9b016c88e6b5570fca777cfcf90a5))
- Automatic Rust CI changes - ([7e01d31](https://github.com/TalpaLabs/coreminer/commit/7e01d31637d9c0abff7aba9dba0a67d7b20d7b35))
- Update to steckrs v0.2.0 - ([baa3fcb](https://github.com/TalpaLabs/coreminer/commit/baa3fcbf92bc6da0f87247c1b27495bac5f4db20))

## [0.2.3](https://github.com/TalpaLabs/coreminer/compare/v0.2.2...v0.2.3)

### ⛰️ Features

- _(cmserve)_ Print a feedback::Error on bad input #38 - ([83c11c7](https://github.com/TalpaLabs/coreminer/commit/83c11c76216f487faba033db3cd006ffcd773c24))
- Log to stderr by default #38 - ([4b1b36d](https://github.com/TalpaLabs/coreminer/commit/4b1b36d69d2942745ade269d5101f56ca0eca264))

### 🐛 Bug Fixes

- _(baseui)_ Argv was set wrong with arguments #39 - ([a32ac8d](https://github.com/TalpaLabs/coreminer/commit/a32ac8d59e10405470463b6cc60f20757864af1e))

## [0.2.2](https://github.com/TalpaLabs/coreminer/compare/v0.2.1...v0.2.2)

### 🐛 Bug Fixes

- _(baseui)_ Argv[0] was not set properly #36 - ([52403f1](https://github.com/TalpaLabs/coreminer/commit/52403f1ec5b3097cd777c7b09c7f5fdb68ed3773))
- _(cmserve)_ Feedback example printed a result - ([cf69a08](https://github.com/TalpaLabs/coreminer/commit/cf69a08ef4b9c378f40c1e8a244b58ef055b295b))

### 📚 Documentation

- Add section about the examples - ([c9f35a4](https://github.com/TalpaLabs/coreminer/commit/c9f35a4e938c0bfed92b4a5460a030e4a92d3ace))
- Fix typo in link of readme - ([d6ff688](https://github.com/TalpaLabs/coreminer/commit/d6ff6889ae43ff798c09d76f899c5de132580661))

### ⚙️ Miscellaneous Tasks

- Automatic Rust CI changes - ([cbddcd0](https://github.com/TalpaLabs/coreminer/commit/cbddcd01e5fe8d181e1d5d90e14fd2fca54b86dc))
- Add print_args example - ([65403d9](https://github.com/TalpaLabs/coreminer/commit/65403d9ff532518e4b882f2fb33ead7787640579))

## [0.2.1](https://github.com/TalpaLabs/coreminer/compare/v0.2.0...v0.2.1)

### 🐛 Bug Fixes

- _(cmserve)_ Allow to use both example flags at once - ([48f6c05](https://github.com/TalpaLabs/coreminer/commit/48f6c05badf103dd11e554139acc1360deb26560))
- Dont panic when dropping the breakpoint fails - ([ace3417](https://github.com/TalpaLabs/coreminer/commit/ace34172f3dcdf3ed2c574a9ba26866a5e0586c7))

### 📚 Documentation

- Add section about cmserve to the readme - ([8a3af52](https://github.com/TalpaLabs/coreminer/commit/8a3af5204a07238687f3e6c6246084370defddaf))

### ⚙️ Miscellaneous Tasks

- Update dummy compile scripts - ([185fb75](https://github.com/TalpaLabs/coreminer/commit/185fb7584340aa48da2596fa6c99f905491508f5))
- Add how_many_fds.c example - ([f5c8beb](https://github.com/TalpaLabs/coreminer/commit/f5c8bebb328d22f96c356982f6e50f425b883593))
- Remove the weird unreleased section from changelog - ([b452be1](https://github.com/TalpaLabs/coreminer/commit/b452be1e8d90b940d0e5a63e01c4c0718ce74e2d))
- Run cargo ci on master but dont commit back - ([8a40169](https://github.com/TalpaLabs/coreminer/commit/8a40169ae6a073297582410b44052a65656a3332))

## [Unreleased]

## [0.2.0](https://github.com/TalpaLabs/coreminer/compare/v0.1.1...v0.2.0)

### ⛰️ Features

- _(basicui)_ Update the help menu and set loglevel to info - ([fbf683a](https://github.com/TalpaLabs/coreminer/commit/fbf683aec6d93694733a3ce9bb3cce71d727c45d))
- _(cmserve)_ Update the help menu - ([ee41636](https://github.com/TalpaLabs/coreminer/commit/ee41636bfc332831d6c71e36da77528a5d7804fc))
- _(cmserve)_ Wrap feedback in a json object and add --example-feedbacks #21 - ([d19569c](https://github.com/TalpaLabs/coreminer/commit/d19569c54db0d28e8a639375872de89285f74fb3))
- Setup human panic for the binaries - ([8fc32af](https://github.com/TalpaLabs/coreminer/commit/8fc32afdb154f31e53bb1caa4615699a50222069))
- Show info for default executable if none was provided but run was used without args - ([3384231](https://github.com/TalpaLabs/coreminer/commit/338423105eb9084b16d6db04c2f3525996de0f59))
- Add input struct for json interface #21 - ([5efc2b2](https://github.com/TalpaLabs/coreminer/commit/5efc2b2ff48c155c65bfa7e8795a046e4b4705f0))
- Read json on \n #21 - ([0b7a785](https://github.com/TalpaLabs/coreminer/commit/0b7a78582680bc57d74acf0ddaa8c7f5585b0189))
- Impl a basic JsonUI::process #16 - ([edf5640](https://github.com/TalpaLabs/coreminer/commit/edf564018ff9ded655e9a446648b8c4f7ceb746c))
- Impl Deserialize for Status #20 #16 - ([03fa022](https://github.com/TalpaLabs/coreminer/commit/03fa022abd86fc3a3fb339b82f5c2c06290bc31e))
- Make Status and Register Serialize #16 #20 - ([20197bd](https://github.com/TalpaLabs/coreminer/commit/20197bd9c99b5523cb60cbc13e777ce014a91e6a))
- JsonUI::process outputs the serialized Feedback #19 #16 - ([3806b6c](https://github.com/TalpaLabs/coreminer/commit/3806b6c73c872ac38a5dc9ebcc6653f6d15ceac7))
- Implement our own ProcessMemoryMap with Serialize #19 - ([9fa0bc4](https://github.com/TalpaLabs/coreminer/commit/9fa0bc40e360a0b4f5af29ceb63961e25971372f))
- Make DebuggerError Serailize #19 - ([c9bed01](https://github.com/TalpaLabs/coreminer/commit/c9bed01e86d7edd758bd7e4e797d48d1a14fd20b))
- Make OwnedSymbol Serialize #19 - ([707e4ad](https://github.com/TalpaLabs/coreminer/commit/707e4ade402a71e531fedb95081a83bf9b56cc8a))
- Make Disassembly Serialize #19 - ([f210b05](https://github.com/TalpaLabs/coreminer/commit/f210b0545137953df06d2b78082f9ec954cdd4a9))
- Make VariableValue Serialize #19 - ([bd14bca](https://github.com/TalpaLabs/coreminer/commit/bd14bca6d99de52b3c0980c2cc17b16919c5024f))
- Make Stack Serialize #19 - ([f870110](https://github.com/TalpaLabs/coreminer/commit/f8701108a635e2abd07ceb0cd29108ef44ec41dc))
- Replace libc::user_regs_struct with UserRegs #19 - ([97d5e60](https://github.com/TalpaLabs/coreminer/commit/97d5e60f0b4d562436fac801c8e466d074e5b576))
- Make Backtrace Serialize #19 - ([9f39092](https://github.com/TalpaLabs/coreminer/commit/9f39092d0a89e1c3b457a9a9108856c593733255))
- Add serde_json error - ([fe0c748](https://github.com/TalpaLabs/coreminer/commit/fe0c7480c4a292fa723f5000385263771504bcdf))
- Make Addr Serialize - ([1cd0abc](https://github.com/TalpaLabs/coreminer/commit/1cd0abc0994387ea339b7f216c10fcb435a0fccc))
- Add basic json interface - ([d4af0d2](https://github.com/TalpaLabs/coreminer/commit/d4af0d2bb8eb92005a492bc54a830e04566cbd19))
- Add cmserve binary - ([0633171](https://github.com/TalpaLabs/coreminer/commit/063317155070ed6738265ec11f50da667abc95c8))

### 🐛 Bug Fixes

- Fix many pedantic warnings and apply some in code - ([bebbc02](https://github.com/TalpaLabs/coreminer/commit/bebbc02ee75f8668ad48e39923e29eb7d148f0f9))
- Json module was not declared - ([987e34d](https://github.com/TalpaLabs/coreminer/commit/987e34d6177cd2a5c08d70132903c256f8b9ecbc))

### 🚜 Refactor

- Fix pedantic warnings - ([f34f45e](https://github.com/TalpaLabs/coreminer/commit/f34f45e7db30214c25d3af2a21372816056ac54c))
- Setup the binaries with less verbose logging - ([4c012bd](https://github.com/TalpaLabs/coreminer/commit/4c012bd37838ddc9f9e9db32a47f5f23de68a72a))
- JsonUI format_feedback is no longer a method - ([9e1dd81](https://github.com/TalpaLabs/coreminer/commit/9e1dd81a0acd0fcfbcace3ff5bd8fc170a243b69))
- Cli build function checks if the given executable is okay, and remove the String field from the Executable errors - ([24c76dd](https://github.com/TalpaLabs/coreminer/commit/24c76ddd5456f0be4902a018c78303bc96ac7aa9))
- Be more clear about when parse_datatype fails - ([86a1afd](https://github.com/TalpaLabs/coreminer/commit/86a1afd62ef1a2c4a3830022933fa7ffa8a53a4a))
- Write_to_line now returns a result instead of panicing on error - ([487f2af](https://github.com/TalpaLabs/coreminer/commit/487f2af1cf0110ac69032c6ec08ef7c0201b56e4))
- Remove unused Feedback variant Text - ([a039a9a](https://github.com/TalpaLabs/coreminer/commit/a039a9a2bf034bdea91b96ff200b8807028169d3))
- Fix some pedantic warnings - ([229d063](https://github.com/TalpaLabs/coreminer/commit/229d063128ad0fae689b89fbf70afcf4c6098254))
- Update Status struct for json interface #21 - ([2270042](https://github.com/TalpaLabs/coreminer/commit/2270042413818c32ab612fcbb6d525045f05ef6c))

### 📚 Documentation

- Doctest was broken from mini-refactoring - ([3acb227](https://github.com/TalpaLabs/coreminer/commit/3acb227cb4098b6d1f5b753d87e45f535acf441c))
- Fix doctests for --no-default-features - ([c87d58c](https://github.com/TalpaLabs/coreminer/commit/c87d58c175caaacfecfb8523cb7e1fb9f052d32c))
- Document json.rs - ([33a2a7b](https://github.com/TalpaLabs/coreminer/commit/33a2a7b7e34f0e320c242ab3451d9d343fd33031))
- Document cli.rs ui module - ([181fd22](https://github.com/TalpaLabs/coreminer/commit/181fd22f8a963b6a477ff8b6c98d57639f79f7a7))
- Fix some examples - ([a3e09a7](https://github.com/TalpaLabs/coreminer/commit/a3e09a7af789b7f93873f1e811250f5813a90d47))
- Document memorymap #21 - ([a4595d6](https://github.com/TalpaLabs/coreminer/commit/a4595d6436469901a54cb428c526e9029bfa77ee))
- Update procmap documentation #21 - ([a68438b](https://github.com/TalpaLabs/coreminer/commit/a68438b90f9b1df013a90febd05328d0f348b8c0))
- Document that OwnedSymbol skips some fields in serialize #21 - ([520d474](https://github.com/TalpaLabs/coreminer/commit/520d474e3508c99c577b77925b03383ff146679b))
- Fix doctest for `get_process_map` #19 - ([2f7eeaa](https://github.com/TalpaLabs/coreminer/commit/2f7eeaa05270f547a5d8c0a07b6acfa06b5a27c6))
- Format api docs in errors - ([e6101cb](https://github.com/TalpaLabs/coreminer/commit/e6101cbc09428cb350a5eb41574ccf012193230c))

### 🧪 Testing

- Disassemble and serialize disassemble #24 - ([4fbd26b](https://github.com/TalpaLabs/coreminer/commit/4fbd26bbca0af46c1fadb9afbe510bc2c1a73ece))
- OwnedSymbol serialization test #24 - ([fe62b6d](https://github.com/TalpaLabs/coreminer/commit/fe62b6d71a787af931eaa2d2b6dfe243a591335b))
- Test_addr_serialize_deserialize for Addr #24 - ([c4cbbeb](https://github.com/TalpaLabs/coreminer/commit/c4cbbeb58660db2c8deaa0b6fe31e7140f8a766a))

### ⚙️ Miscellaneous Tasks

- Configure release-plz - ([b722cf1](https://github.com/TalpaLabs/coreminer/commit/b722cf156bd9897d35e18aee042e30e94acf252e))
- Dont run cargo ci on master (commit back is disallowed) - ([1ca92ea](https://github.com/TalpaLabs/coreminer/commit/1ca92ea8ef15587fc74638c6f1b2fe16b32d3d1d))
- Automatic Rust CI changes - ([2d6df9e](https://github.com/TalpaLabs/coreminer/commit/2d6df9e2f77e9b041063f262e8bca8cc04750f08))
- Add features to coreminer to keep things more organized - ([bac2dca](https://github.com/TalpaLabs/coreminer/commit/bac2dca74ee283060a6743a6665dd08522062212))
- Ci now tests with --no-default-features too - ([c25d71d](https://github.com/TalpaLabs/coreminer/commit/c25d71d693e7b5c4cdb7e9c1918d74cac3633252))
- Remove unused dependency addr2line - ([c8c3324](https://github.com/TalpaLabs/coreminer/commit/c8c332443bd394c6fc19082650d586ab78222e40))
- Remove unused dependency ouroboros - ([4584ae0](https://github.com/TalpaLabs/coreminer/commit/4584ae02855ba8e3c0d65c20d39ca2720a8e3e01))
- Disable some uneeded warnings - ([94ddd14](https://github.com/TalpaLabs/coreminer/commit/94ddd14d077d9c334d8ab9d71f54f28fef189919))
- Make clippy super pedantic and comlplain about docs - ([94be3d8](https://github.com/TalpaLabs/coreminer/commit/94be3d8c19977acfae5a4cf91550439e49c807bb))
- JsonUI::process is still a todo - ([e796b0a](https://github.com/TalpaLabs/coreminer/commit/e796b0a56631825aaa12f310bd2f6407dffc52b5))
- Warn on missing docs - ([7e33980](https://github.com/TalpaLabs/coreminer/commit/7e339806c28a07fa9bc4f978f0e9e525903c5ea8))

# Changelog

All notable changes to this project will be documented in this file.

## [0.1.1](https://github.com/TalpaLabs/coreminer/compare/v0.1.0...v0.1.1) - 2025-02-28

### Other

- change docs.rs links to go to the documentation directly
- change second emoji in readme because it's not displayed correctly on some devices (#14)
- fix github actions links in readme
- change links from PlexSheep/repo to organization
- set changelog

## [0.1.0] - 2025-02-26

### 🚀 Features

- Launch the debuggee
- Super basic debugger interface
- Feedback to ui
- Breakpoint struct
- Add breakpoints to debugee
- Breakpoints (setting) works
- Remove breakpoints
- Improve the basic cli interface with dialoguer
- Feedback error handling
- Set registers
- Rmem and wmem
- Step over breakpoint
- Get debug data from executable
- Process map
- Early disassemble
- Disassembly looks okay
- Disassembly datastructure
- Read any length from memory
- Write any length to process memory
- Set len for disassembly in debug-cli
- Debug symbols for functions
- Read which function we're in right now
- Find important constants
- Wait_signal
- Single step
- We think step over works
- Step over
- Only wait for interesting signals
- Add raw data to disassembly
- Don't allow stepping out of main if we know it
- Step into
- Step over
- Backtrace with libunwind
- Preparse all gimli symbols with a tree structure
- Query symbol tree
- Parse more symbol kinds
- Read types from dwarf ifno (just the usize)
- Get type for symbol
- Work on reading location expressions
- Eval expression maybe works
- We can read the location of an example
- Gimli frame_base parsing
- Impl custom debug for OwnedSymbol
- Read variable but wrong
- Stack datastructure
- Read stack
- Stop waiting on SIGWINCH
- Write variable with debug symbol
- Pass process maps to the ui
- Always check if the child is exited in functions that return a feedback
- Hide int3 instructions from disassembly (unless explicitly wished), add breakpoints to disassembly
- _(baseui)_ Add default executable to base ui

### 🐛 Bug Fixes

- Create cstring with CString::new instead of from_str
- Some commands in the debug cli did not use get_number
- Step over breakpoint at cont
- Breakpoint inverse mask was wrong
- Log ignored signals and finish waiting on SIGILL
- Fix the step out SIGSEGV
- Log if go_back_step_over_bp actually does something
- Addresses for dwarf were wrongly parsed
- Addr debug didnt use hex
- Debug of addr had wrong format
- Stack addresses were displayed wrong
- Read variable reads an older version of the variable stored somewhere else???
- Read variable hack
- Wmem debug ui had wrong index
- Catch the exit status of the debuggee in wait_status
- Fill_to_const_arr did not use the internal vec
- Regs set parsing was broken in testing ui
- Set_bp and del_bp still used unwrap to get the debuggee
- Step_out used an unwrap to get the debuggee

### 🚜 Refactor

- Cli starts_with_any
- Move addr and add wmem+rmem
- The debuginfo loader
- Generalize debug symbols
- Impl Display for Disassembly
- Dse is here to stay (and maybe buggy)
- Use the gimli EntriesTree
- FrameInfo struct added
- Do not evaluate dwarf expressions at pre loading
- OwnedSymbol constructor change, read byte_size for types
- Rename parse_byte_site to parse_udata
- Error handling for variable reading logic
- Move addr to it's own module
- Addr module now works more with usize and has more traits
- Run any executable interactively
- Remove check_debuggee_status
- Remove unneeded fields and functions
- Variable access has less code duplication
- Entry_from_gimli is now much simpler without the large match
- Remove the prologue detection in step-in
- Remove the Addr::from_relative method, as it's just an addition
- Remove Addr::relative as it's just a subtraction
- Debuggee::get_symbol_by_offset does not panic when multiple matches are found, instead returns an error
- Remove old debug prints in run_debugger
- Remove unused method in dwarf_parse
- _(baseui)_ Generally improve the baseui with error handling and a help menu

### 📚 Documentation

- Add a basic readme
- Api documentation for lib.rs
- Document the addr module
- Document the breakpoint module
- Document consts module
- Document the dbginfo module
- Document the debuggee module
- Amend enable and disable documentation of breakpoint with additional error reasons
- Document debugger module
- Fix doctests in debugger
- Document the disassemble module
- Fix a warning
- Ackowledge bugstalker for not just unwinding
- Document dwarf_parse module
- Remove example for private function
- Document errors module
- Document feedback module
- Document stack module
- Document the ui module
- Document unwind module
- Document the remaining core modules
- Adjust readme for changes to the baseui
- Add keywords and categories
- Fancy readme with logo and links
- Fix doctests, CliUi::build was broken

### 🧪 Testing

- Add tests for addr
- Tests for variablevalue
- Add test for stack operations
- Add tests for dbginfo

### ⚙️ Miscellaneous Tasks

- Setup basic project
- Add some deps which we probably need
- Automatic Rust CI changes
- Fix typo debugee -> debuggee
- Automatic Rust CI changes
- Remove uneeded part in cargo.yaml ci
- Add example dummy c script to debug
- Automatic Rust CI changes
- Build example dummy with debug info
- Add fixme to ptrace::step
- Automatic Rust CI changes
- Automatic Rust CI changes
- Automatic Rust CI changes
- Automatic Rust CI changes
- Automatic Rust CI changes
- Our debugger half works :)
- Automatic Rust CI changes
- Automatic Rust CI changes
- Better dummy compile scripts
- Install system deps in ci
- Automatic Rust CI changes
- Automatic Rust CI changes
- Automatic Rust CI changes
- Add dummy3.c
- Automatic Rust CI changes
- Automatic Rust CI changes
- Remove comment that is no longer relevant
- Rename a test in breakpoint
- Fix build-release script
- Rust ci now runs the doctests
- Allow publishing of coreminer
- Add msrv
- Create empty CHANGELOG
- Enforce maximum keywords limit
- Setup git-cliff
- Setup dependabot for cargo

<!-- generated by git-cliff -->
