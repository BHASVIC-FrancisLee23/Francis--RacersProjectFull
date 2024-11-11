import pygame
from pygame import *
from time import sleep

background_colour = (25,25,25)
RED =       (255,   0,   0)
(width, height) = (600, 375)
screen = pygame.display.set_mode((width, height))
pygame.display.set_caption('Track Creation Demonstation')
screen.fill(background_colour)
pygame.display.flip()

points = [(242, 75), (364, 90), (458, 164), (450, 250), (335, 288), (241, 282), (104, 264), (93, 154), (141, 97)]
delay = 300
index = 0
drawingPoints = True
time1 = 0
w = 2
pointDrawn = False
delay1 = 0

running = True
while running:
  screen.fill(background_colour)
  

  if index == len(points):
    # draw lines
    for i in range(0, len(points) - 1):
      pygame.draw.line(screen, "Green", points[i], points[i + 1], w)
    pygame.draw.line(screen, "Green", points[-1], points[0], w)
    delay1 += 1


  if drawingPoints:
    for i in range(0, index):
      print(i)
      pygame.draw.circle(screen, Color(255, 0, 0,), points[i], 4)


  for event in pygame.event.get():
    if event.type == pygame.QUIT:
      running = False

  if time1 % delay == 0:
    index += 1
  if index > len(points):
    index = len(points)

  time1 += 1

  if delay1 > 500:
    w += 1
    if w > 40:
        w  = 40

  pygame.display.update()
