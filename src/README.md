# Handling Data in Minerust

**********

**Database of choice for storing a chunk**<br>
The database of choice is Sled, but how will storing data look?

Lets first look at what a chunk is.<br>
- A chunk is a 16x16 piece of land of an infinite height
- A chunk's location is designated by its distance (int) from the (0,0) chunk, i.e: left from (0,0) is (1,0)

How does a chunk look? 

A chunk is a 2d 16x16 array, wrapped by a vector. The vector allows for an infinite build height

***

- There are two sled trees:<br/>
    1. A tree for chunks: Database Key: Vec2 of the asigned coordinate<br/>
    2. A tree for blocks: Database key: Vec3 location of block

****
