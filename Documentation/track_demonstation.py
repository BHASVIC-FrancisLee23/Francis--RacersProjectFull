import pygame
from pygame import *
from time import sleep
import math

background_colour = (25,25,25)
RED =       (255,   0,   0)
(width, height) = (600, 375)
width *= 1.5
height *= 1.5
screen = pygame.display.set_mode((width, height))
pygame.display.set_caption('Track Creation Demonstation')
screen.fill(background_colour)
pygame.display.flip()

points = [[242, 75], [364, 90], [458, 164], [450, 250], [335, 288], [241, 282], [104, 264], [93, 154], [141, 97]]
for p in points:
  p[0] *= 1.5
  p[1] *= 1.5

print(points[0])
delay = 300
index = 0
drawingPoints = True
time1 = 0
w = 2
pointDrawn = False
delay1 = 0


def drawLine(x1, y1, x2, y2, w, steps = 20):
  vector = (x2 - x1, y2 - y1)
  for step in range(20):
    pygame.draw.circle(screen, "Green", (x1 + step * (vector[0] / steps), y1 + step * (vector[1] / steps)), w/2)


running = True
while running:
  screen.fill(background_colour)
  
  if index == len(points):
    # draw lines
    for i in range(0, len(points) - 1):
      pygame.draw.line(screen, "Green", points[i], points[i + 1], w)
      drawLine(points[i][0], points[i][1], points[i + 1][0], points[i+1][1], w)
    pygame.draw.line(screen, "Green", points[-1], points[0], w)
    drawLine(points[-1][0], points[-1][1], points[0][0], points[0][1], w)
    delay1 += 1


  if drawingPoints:
    for i in range(0, index):
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
    if time1 % 5 == 0:
      w += 1
      if w > 50:
          w = 50


  pygame.display.update()


