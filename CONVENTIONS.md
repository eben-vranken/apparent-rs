## Conventions
This is a document that will help me keep track of all design choices, so that I don't make mistakes that contradict my irreversible decisions.

---

#### Degrees
1. Angles are radians internally. Degrees/hours only at print and parse.
Gaia catalogue only uses degrees so I have no choice to parse them, radians for printing would be messy and not easily understood.

#### Frames and directions

2. Rotations are passive (I move the frame), matching the SOFA guidelines.
3. Azimuth is measured from north, increasing through east ([0, 2π))
4. Longitude is east-positive