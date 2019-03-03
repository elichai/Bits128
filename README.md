# Bits128
Iterating over 128 array of bytes


`bits128` provides a struct that let's you use 128 bits while taking only 128 bits in memory. <br>
if you would use something like `[bool; 128]` it would take 128*8 bits in memory because every bool takes 1 bytes(8bits) <br>
In the future I'll implement an Iterator over the bits so you can iterate over them easily. <br>