import pygame
import math
import random
import time
import sys

# Initialize Pygame
pygame.init()

# Game constants
WIDTH, HEIGHT = 800, 600
FPS = 60
BULLET_COOLDOWN = 0.5  # seconds
ZOMBIES_PER_WAVE = 4

# Colors
WHITE = (255, 255, 255)
RED = (255, 0, 0)
BLACK = (0, 0, 0)

# Setup the display
screen = pygame.display.set_mode((WIDTH, HEIGHT))
pygame.display.set_caption("Shooter Game")

# Load zombie sprite
zombie_img = pygame.image.load("skibidi.png").convert_alpha()
zombie_img = pygame.transform.scale(zombie_img, (50, 50))

zombie_speed = 1

# Game variables
levels = 0
kills = 0

# Player class
class Player(pygame.sprite.Sprite):
    def __init__(self):
        super().__init__()
        self.original_image = pygame.Surface((40, 40), pygame.SRCALPHA)
        pygame.draw.polygon(self.original_image, WHITE, [(20, 0), (0, 40), (40, 40)])

        self.image = self.original_image.copy()
        self.rect = self.image.get_rect(center=(WIDTH - 50, HEIGHT // 2))
        self.speed = 5
        self.last_shot_time = 0

    def handle_input(self, keys, mouse_pos):
        # Movement
        if keys[pygame.K_w] and self.rect.top > 0:
            self.rect.y -= self.speed
        if keys[pygame.K_s] and self.rect.bottom < HEIGHT:
            self.rect.y += self.speed
        if keys[pygame.K_a] and self.rect.left > 0:
            self.rect.x -= self.speed
        if keys[pygame.K_d] and self.rect.right < WIDTH:
            self.rect.x += self.speed
        if keys[pygame.K_SPACE]:
            self.shoot()

        # Rotation
        dx, dy = mouse_pos[0] - self.rect.centerx, mouse_pos[1] - self.rect.centery
        angle = math.degrees(math.atan2(-dy, dx)) - 90
        self.image = pygame.transform.rotate(self.original_image, angle)
        self.rect = self.image.get_rect(center=self.rect.center)

    def shoot(self):
        current_time = time.time()
        if current_time - self.last_shot_time > BULLET_COOLDOWN:
            channel = pygame.mixer.Channel(1)
            sound = pygame.mixer.Sound('sound/bombastic-boom.mp3')
            channel.set_volume(0.2)
            channel.play(sound)
            
            bullet = Bullet(self.rect.centerx, self.rect.centery, pygame.mouse.get_pos())
            all_sprites.add(bullet)
            bullets.add(bullet)
            self.last_shot_time = current_time


# Bullet class
class Bullet(pygame.sprite.Sprite):
    def __init__(self, x, y, target):
        super().__init__()
        self.image = pygame.Surface((5, 10))
        self.image.fill(RED)
        self.rect = self.image.get_rect(center=(x, y))
        self.speed = 10
        self.target = target
        dx, dy = target[0] - x, target[1] - y
        distance = math.hypot(dx, dy)
        self.velocity = (dx / distance * self.speed, dy / distance * self.speed)

    def update(self):
        self.rect.x += self.velocity[0]
        self.rect.y += self.velocity[1]
        if self.rect.right < 0 or self.rect.left > WIDTH or self.rect.bottom < 0 or self.rect.top > HEIGHT:
            self.kill()

# Zombie classx
class Zombie(pygame.sprite.Sprite):
    def __init__(self):
        super().__init__()
        self.image = zombie_img
        self.rect = self.image.get_rect(center=(0, random.randint(0, HEIGHT)))
        self.health = 100

    def update(self):
        global kills
        player_pos = player.rect.center
        dx, dy = player_pos[0] - self.rect.centerx, player_pos[1] - self.rect.centery
        distance = math.hypot(dx, dy)
        self.rect.x += dx / distance * zombie_speed
        self.rect.y += dy / distance * zombie_speed

        bullet = pygame.sprite.spritecollideany(self, bullets)
        if bullet:
            bullet.kill()

            self.health -= 50
            if self.health <= 0:
                # play zombie death sound
                pygame.mixer.Channel(0).play(pygame.mixer.Sound('sound/rahwr.mp3'))
                kills += 1
                self.kill()



# Initialize sprites
bullets = pygame.sprite.Group()
zombies = pygame.sprite.Group()

all_sprites = pygame.sprite.Group()
player = Player()
all_sprites.add(player)

# Game loop variables
clock = pygame.time.Clock()

# Keeps track of how many seconds to wait before spawning next zombie
zombie_spawn_time_wait = 2
zombie_spawn_timer = 1

zombies_spawned_in_wave = 0

running = True
game_started = False
game_start_time = None


# PLAY BACKGROUND MUSIC that loops and is on 50% volume sound/skibiditoilet.mp3
pygame.mixer.music.load('sound/skibiditoilet.mp3')
pygame.mixer.music.play(-1)
pygame.mixer.music.set_volume(0.5)

# Start screen loop
while not game_started:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            sys.exit()
        elif event.type == pygame.KEYDOWN and event.key == pygame.K_SPACE:
            game_start_time = time.time()
            game_started = True

    clock.tick(FPS)
    screen.fill(BLACK)

    font = pygame.font.Font(None, 74)
    text = font.render("Press SPACE to start", True, WHITE)
    screen.blit(text, (WIDTH // 2 - text.get_width() // 2, HEIGHT // 2))
    
    pygame.display.flip()


# Main game loop
while running:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            sys.exit()

    # Get input
    keys = pygame.key.get_pressed()
    mouse_pos = pygame.mouse.get_pos()
    
    # Update player manually
    player.handle_input(keys, mouse_pos)
    
    # Spawn zombies
    if zombie_spawn_timer <= 0:
        zombies_spawned_in_wave += 1

        zombie = Zombie()
        all_sprites.add(zombie)
        zombies.add(zombie)

        if zombies_spawned_in_wave == ZOMBIES_PER_WAVE:
            zombies_spawned_in_wave = 0
            levels += 1

            if zombie_spawn_time_wait == 1:
                zombie_speed += 0.25
            else:
                zombie_spawn_time_wait -= 0.5

        zombie_spawn_timer = zombie_spawn_time_wait
    else:
        zombie_spawn_timer -= 1 / FPS

    # Update all other sprites
    all_sprites.update()

    # Check for collisions
    if pygame.sprite.spritecollideany(player, zombies):
        channel = pygame.mixer.Channel(2)
        sound = pygame.mixer.Sound('sound/getout.mp3')
        channel.set_volume(0.2)
        channel.play(sound)

        running = False

    clock.tick(FPS)

    # Draw everything
    screen.fill(BLACK)
    all_sprites.draw(screen)

    pygame.display.flip()


# play game over sound, as sound

# wait 1 sec
time.sleep(1)
pygame.mixer.music.load('sound/siren.mp3')
pygame.mixer.music.play(-1)

# Game over screen
while True:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            sys.exit()

    clock.tick(FPS)
    screen.fill(BLACK)

    font = pygame.font.Font(None, 74)
    text = font.render(f"Game Over! Score: {kills}", True, WHITE)
    screen.blit(text, (WIDTH // 2 - text.get_width() // 2, HEIGHT // 2 - text.get_height() // 2))
    pygame.display.flip()

# pygame.quit()
