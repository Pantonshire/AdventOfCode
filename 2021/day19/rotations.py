def rot_x(c):
    return (c[0], c[2], -c[1])

def rot_y(c):
    return (c[2], c[1], -c[0])

def rot_z(c):
    return (c[1], -c[0], c[2])

def coord_to_s(c):
    return ("Coord::new(" + ", ".join([str(x) for x in c]) + "),").replace("1", "self.x").replace("2", "self.y").replace("3", "self.z")

rots = {}
for x in range(4):
    for y in range(4):
        for z in range(4):
            p = (1,2,3)
            for i in range(x):
                p = rot_x(p)
            for i in range(y):
                p = rot_y(p)
            for i in range(z):
                p = rot_z(p)
            rots[p] = True

for rot in rots:
    print(coord_to_s(rot))
