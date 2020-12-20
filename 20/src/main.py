import math

with open("input", "r") as fd:
  contents = fd.read()

parts = contents.split("\n\n")
del contents

def flip_h(tile):
  return (tile[0], [r[::-1] for r in tile[1]])

def flip_v(tile):
  return (tile[0], tile[1][::-1])

def rotate(tile):
  return (tile[0], [list(r) for r in zip(*tile[1][::-1])])

def rotations(tile):
  r1 = rotate(tile)
  r2 = rotate(r1)
  return [tile, r1, r2, rotate(r2)]

def transformations(tile):
  return rotations(tile) + rotations(flip_h(tile)) + rotations(flip_v(tile))

def matches_h(left, right):
  c1 = [r[-1] for r in left[1]]
  c2 = [r[0] for r in right[1]]
  return c1 == c2

def matches_v(top, bottom):
  return top[1][-1] == bottom[1][0]

def copy_tile(tile):
  return (tile[0], [[x for x in r] for r in tile[1]])

def copy_grid(grid):
  return [[None if t == None else copy_tile(t) for t in row] for row in grid]

def find_arrangement(x, y, grid, used):
  l = len(grid)
  next = ((x+1)%l, y+((x+1)//l)) if x < l-1 or y < l-1 else None
  left = grid[y][x-1] if x > 0 else None
  top = grid[y-1][x] if y > 0 else None
  ks = []
  if x == 0 and y == 0:
    ks = [(3023, i) for (i,_) in enumerate(ts[3023])]
  elif x == l-1 and y == 0:
    ks = [(1571, i) for (i,_) in enumerate(ts[1571])]
  elif x == 0 and y == l-1:
    ks = [(1709, i) for (i,_) in enumerate(ts[1709])]
  elif x == l-1 and y == l-1:
    ks = [(3457, i) for (i,_) in enumerate(ts[3457])]
  elif left != None and top != None:
    ks = [k for k in rcs[left] if k in bcs[top] and not (k[0] in used)]
  elif left != None:
    ks = [k for k in rcs[left] if not (k[0] in used)]
  elif top != None:
    ks = [k for k in bcs[top] if not (k[0] in used)]
  else:
    ks = [k for k in acs if not (k[0] in used)]
  for k in ks:
    grid[y][x] = k
    if next == None:
      return grid
    used_c = used.copy()
    used_c[k[0]] = True
    g = find_arrangement(next[0], next[1], grid.copy(), used_c)
    if g != None:
      return g
  return None

tiles = [[lt for lt in (l.strip() for l in p.split("\n")) if lt] for p in parts]
tiles = [(int(t[0][len("Tile "):][:-1]), [[c == '.' for c in l] for l in t[1:]]) for t in tiles]

grid_l = int(math.sqrt(len(tiles)))

ts = {}
for t in tiles:
  ts[t[0]] = transformations(t)

acs = []
rcs = {}
bcs = {}
for t1 in tiles:
  for (i,t1_t) in enumerate(ts[t1[0]]):
    k = (t1[0], i)
    acs.append(k)
    rcs[k] = []
    bcs[k] = []
    for t2 in tiles:
      for (j,t2_t) in enumerate(ts[t2[0]]):
        if matches_h(t1_t, t2_t):
          rcs[k].append((t2[0], j))
        if matches_v(t1_t, t2_t):
          bcs[k].append((t2[0], j))

grid = [[None for _ in range(grid_l)] for _ in range(grid_l)]
grid = find_arrangement(0, 0, grid, {3023: True, 1571: True, 1709: True, 3457: True})

img = []
for row in grid:
  r = [[] for _ in range(8)]
  for tile in row:
    tile = ts[tile[0]][tile[1]]
    for (i,x) in enumerate(tile[1][1:-1]):
      r[i].extend(x[1:-1])
  for s in r:
    img.append(s)

for i in range(len(img)):
  img[i] = ''.join(['.' if x else '#' for x in img[i]])

sea_monster = [
  "                  # ",
  "#    ##    ##    ###",
  " #  #  #  #  #  #   ",
]

sea_monster = [[c == '#' for c in r] for r in sea_monster]
sea_monster_h = len(sea_monster)
sea_monster_w = len(sea_monster[0])

def img_flip_h(im):
  return [r[::-1] for r in im]

def img_flip_v(im):
  return im[::-1]

def img_rotate(im):
  return [list(r) for r in zip(*im[::-1])]

def img_rotations(im):
  r1 = img_rotate(im)
  r2 = img_rotate(r1)
  return [im, r1, r2, img_rotate(r2)]

def img_transformations(im):
  return img_rotations(im) + img_rotations(img_flip_h(im)) + img_rotations(img_flip_v(im))

for img_t in img_transformations(img):
  n = 0
  for i in range(len(img_t) - sea_monster_h + 1):
    for j in range(len(img_t[0]) - sea_monster_w + 1):
      is_monster = True
      for x in range(sea_monster_h):
        for y in range(sea_monster_w):
          if sea_monster[x][y] and img_t[i+x][j+y] != '#':
            is_monster = False
      if is_monster:
        n += 1
  if n > 0:
    break

s = ''.join([''.join(r) for r in img])
print("Part 2: " + str(len([c for c in s if c == '#']) - 15 * n))
