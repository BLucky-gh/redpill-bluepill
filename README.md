# Redpill-Bluepill

This is a simulation of the following thought experiment that was the subject of some twitter drama recently

```
there's a bunch of people in the room, each one is presented with a choice of blue and red pill,
if more than half of the people choose the blue pill, everyone lives
if more than half of the people choose the red pill, the ones who chose the blue pill die
```

I decided to simulate it for fun because I had nothing better to do, and because I recently saw [this](https://ncase.me/trust/) and wanted to do something like it

## Findings

After messing around with it for a bit I found that as long as the number of `Guesser`s is less than double the difference between `AlwaysBlue`s and `AlwaysRed`s, and there are more `AlwaysBlue`s than `AlwaysRed`s, no one dies regardless of the number of iterations performed, do with that information what you will, I just thought it's neat
