import math

class Solution:
    def checkOverlap(self, radius: int, xCenter: int, yCenter: int, x1: int, y1: int, x2: int, y2: int) -> bool:
        # Translate all points so that circle center is at origin
        x1 -= xCenter
        y1 -= yCenter
        x2 -= xCenter
        y2 -= yCenter
        xCenter, yCenter = 0, 0

        # Check if 4 corners inside circle
        if any([math.dist([x, y], [xCenter, yCenter]) <= radius for x in [x1, x2] for y in [y1, y2]]):
            return True
        
        # Check if any edge touch
        if y1 < 0 and y2 > 0 and (abs(x1) <= radius or abs(x2) <= radius):
            return True
        
        if x1 < 0 and x2 > 0 and (abs(y1) <= radius or abs(y2) <= radius):
            return True

        # Check if circle center within square
        if x1 < 0 and x2 > 0 and y1 < 0 and y2 > 0:
            return True

        return False