## Conventions
This is a document that will help me keep track of all design choices, so that I don't make mistakes that contradict my irreversible decisions.

---

#### Angles
1. Angles are radians internally. Degrees/hours only at print and parse.
Gaia catalogue only uses degrees so I have no choice to parse them, radians for printing would be messy and not easily understood.
2. normalize_2pi returns [0, 2π), normalize_pi returns [-π, π).
Both include the low end and exclude the high end, so I only have to remember one rule instead of two. Exactly half a turn comes back as -π, never +π.

#### Frames and directions

1. Rotations are passive (I move the frame), matching the SOFA guidelines.
2. In a chain of rotations the rightmost matrix applies first, so I read them right to left.
Same kind of silent bug as passive vs active, and just as easy to get backwards.
3. Azimuth is measured from north, increasing through east ([0, 2π))
4. Longitude is east-positive

#### Celestial Frames
1. **z** points at the north celestial pole
2. **x** points where the celestial equator crosses RA 0, the equinox direction
3. **y** completes a right-handed set: RA 90° on the equator