<div align="center">
  <img src=".github/demo.png" width="75%" height="75%"/>
  <h1>PixelPal</h1>
  <h3>Play with ChatGPT!</h3>

  [![License](https://img.shields.io/github/license/chaoxel/PixelPal?style=for-the-badge&logo=github)](LICENSE)
  [![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/chaoxel/PixelPal/ci.yml?style=for-the-badge)](https://github.com/chaoxel/PixelPal/actions)
  [![Discord](https://img.shields.io/discord/1195177036210253864?color=5865F2&label=discord&style=for-the-badge)](https://discord.gg/jwb26Xy5M7)

</div>

## Description
This project, currently in progress, integrates GPT-4 into a game, where it serves as a dynamic and helpful in-game companion.  
Powered by Rust and Bevy, it's a modern and exciting adventure waiting to unfold.

## Setup & Running

### Prerequisites
- [ChatGPT API Key](https://platform.openai.com/)
- [Rust](https://www.rust-lang.org/) (building)
- [Cargo](https://www.rust-lang.org/) (building)

### Step 1: Clone the Repository

Clone this repository to your local machine using Git.

```bash
git clone https://github.com/chaoxel/PixelPal.git
```

### Step 2: Setup Your API Key

Navigate to the project directory and open the `.env.example` file.  
Add your API key as shown below:

```bash
# YOUR KEY HERE
GPT_KEY=sk-aaSWdEOk...
```

After adding the key, rename the file from `.env.example` to `.env`. 

### Step 3: Compile & Run

You can now compile and run the project.
```bash
cargo run --release
```

## License
Licensed under AGPL (Affero General Public License).
