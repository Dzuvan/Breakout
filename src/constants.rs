pub const SCREEN_WIDTH:i32  = 800;
pub const SCREEN_HEIGHT:i32  = 600;

pub const FRAMES_PER_SECONDS: i32 = 30;
pub const FRAME_RATE: i32 = 1000 / FRAMES_PER_SECONDS;

// Location of images within bitmap.
pub const PADDLE_BITMAP_X: i32 = 0;
pub const PADDLE_BITMAP_Y: i32 = 0;

pub const BALL_BITMAP_X: i32 = 100;
pub const BALL_BITMAP_Y: i32 = 0;

pub const YELLOW_X: i32 = 0;
pub const YELLOW_Y: i32 = 20;

pub const RED_X: i32 = 0;
pub const RED_Y: i32 = 40;

pub const BLUE_X: i32 = 80;
pub const BLUE_Y: i32 = 20;

pub const GREEN_X: i32 = 80;
pub const GREEN_Y: i32 = 40;

// Minmum distance from the side of the screen to a block.
pub const BLOCK_SCREEN_BUFFER: i32  = 40;

// Maximum number of block allowed.
pub const MAX_BLOCKS: i32 = 80;

// Number of rows and columns of blocks.
pub const NUM_ROWS: i32 = 6;
pub const NUM_COLS: i32 = 9;

// Location of paddle in the game:
pub const PLAYER_Y: i32 = 550;

// Dimensions of a paddle.
pub const PADDLE_WIDTH: i32 = 100;
pub const PADDLE_HEIGHT: i32 = 20;

// Dimensions of a block.
pub const BLOCK_WIDTH: i32 = 80;
pub const BLOCK_HEIGHT: i32 = 20;

// Ball diameter.
pub const BALL_DIAMETER: i32 = 20;

// Paddle speed.
pub const PLAYER_SPEED: i32 = 10;

// Ball speeds.
pub const BALL_SPEED_MODIFIER: i32 = 5; // Divide location of paddle by this.
pub const BALL_SPEED_Y: i32 = 10;   // Max speed along y axis.

// Maximum number of lives player has a.k.a ball misses player can have.
pub const NUM_LIVES: i32 = 5;

// Number of levels.
pub const NUM_LEVELS: i32 = 3;

// Locations of output text.
pub const LIVES_X: i32 = 5;
pub const LIVES_Y: i32 = 5;
pub const LEVEL_X: i32 = 75;
pub const LEVEL_Y: i32 = 5;
