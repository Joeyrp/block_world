# Block World Demo
This is a demo project I used to learn about OpenGL in Rust. It is a basic minecraft-style world chunk generator. There are 5 different types of noise algorithms implemented: Random 2D, Random 3D, OLC (A version of perlin-like noise implemented by [Youtuber Javidx9](https://www.youtube.com/javidx9) in [this video](https://www.youtube.com/watch?v=6-0UaeJBumA)), Simplex 2D and Simplex 3D. Various parameters of these noise algorithms can be changed on the fly so you can see their affect in real time.

## Note about the code
I was (and still am really) pretty new to Rust when I wrote this. The code is not commented very well so I appologize for that if you're looking through the code. The code is also poorly written/organized in several places and while I'd love to fix it, I think I'd rather just work on a new project. But I'll leave this here for archival purposes.

## Demo Usage
The instructions for interacting with the demo will appear on screen and change based on which noise algorithm is selected.

## Exampe Screen Shot
![demo_screen](https://raw.githubusercontent.com/joeyrp/block_world/master/assets/block_world_scrn1.png)