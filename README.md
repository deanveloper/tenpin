This is my first real rust library. Being as such, I am not sure what the best coding practices in Rust are, so hopefully this works well. Pull requests are welcome!

# Tenpin

Tenpin is a ten pin bowling points manager written in Rust.

Some important definitions:
| Term | Brief Definition |
| --- | --- |
| Bowl | A single instance of throwing the ball |
| Frame | A "turn" on a scorecard, usually consists of 2 bowls, or 1 bowl on a strike |
| Open Frame | A frame where not all pins were knocked down |
| Closed Frame | A frame where all 10 pins were knocked down |
| Spare | A closed frame where it took two bowls to knock all 10 pins down |
| Strike | A closed frame where all 10 pins were knocked down on the first bowl |

Each player bowls 10 frames.

A standard frame:
 * Each frame consists of at most two bowls, except the 10th frame which _may_ contain 3 bowls.
 * If someone knocks down all 10 pins in the first throw (called a "strike", denoted "X"), the number of pins for the next two bowls gets added onto this frame's score.
 * If someone knocks down all 10 pins by the second throw (called a "spare", denoted "/"), the number of pins knocked down in the next bowl gets added onto this frame's score.

Because a strike or spare's point value depends on the following bowls, the final frame has some special rules:

The 10th frame:
 * If someone rolls a strike, they get 2 extra bowls. These bowls do not count for points on their own, and are only used to calculate the bonus points for the strike. This totals for 3 bowls on the 10th frame.
 * Similarly, if someone rolls a spare, they get one extra bowl. This totals for 3 bowls on the 10th frame.
 * Otherwise if the frame is left open (not a strike or spare), then the frame ends after the second bowl.
