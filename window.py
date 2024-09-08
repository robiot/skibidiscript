import pygame
import math

# Initialize pygame
pygame.init()

# Constants
SCREEN_WIDTH = 800
SCREEN_HEIGHT = 600
PLAYER_SIZE = 50
WHITE = (255, 255, 255)
BLUE = (0, 0, 255)
FPS = 60

# Setup the screen
screen = pygame.display.set_mode((SCREEN_WIDTH, SCREEN_HEIGHT))
pygame.display.set_caption("Player Movement and Rotation")

# Clock for controlling frame rate
clock = pygame.time.Clock()

# Player variables
player_x = SCREEN_WIDTH / 2
player_y = SCREEN_HEIGHT / 2
player_angle = 0
player_speed = 5

# Create player surface
player_original_image = pygame.Surface((PLAYER_SIZE, PLAYER_SIZE), pygame.SRCALPHA)
pygame.draw.polygon(player_original_image, BLUE, [(PLAYER_SIZE / 2, 0), (PLAYER_SIZE, PLAYER_SIZE), (0, PLAYER_SIZE)])

# Main game loop
running = True
while running:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False

    keys = pygame.key.get_pressed()

    # Rotate left and right
    if keys[pygame.K_LEFT]:
        player_angle += 5
    if keys[pygame.K_RIGHT]:
        player_angle -= 5

    # Move forward
    if keys[pygame.K_UP]:
        rad = math.radians(player_angle)
        dx = player_speed * math.sin(rad)
        dy = player_speed * math.cos(rad)
        player_x += dx
        player_y -= dy

    # Update the image to reflect rotation
    player_image = pygame.transform.rotate(player_original_image, player_angle)
    player_rect = player_image.get_rect(center=(player_x, player_y))

    # Clear screen
    screen.fill(WHITE)

    # Draw player
    screen.blit(player_image, player_rect.topleft)

    # Refresh display
    pygame.display.flip()

    # Cap the frame rate
    clock.tick(FPS)

pygame.quit()