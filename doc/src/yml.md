# overview

We define tags, which are string values.

The animation is a list of frames. For each frame, we each tag has one value. 
But, because we don't want to define again, for each frame, the value of each tag, we only define the tags that change.

# yml file
we have these keys :

    - delay : the delay in ms between two frames
    - tags : a list of possible tags
    - variants : a list of possible ( `name`, `value`) pairs
    - frames : a list of frames

A frame has keys :

    - title
    - toggles : a list of string, of the form `tag`@`variant name`


## Example : git linear commit diagram.

[see the yaml file](./git-animate.yml)

### tags
we want to animate, eg change the colors of the nodes and eges of the diagram, so we define a tag for each of these items :

```yml
{{#include git-animate.yml:tags}}
```

in this example, `m<i>` ... are the nodes for the main branch, `d<i>` the nodes for the develop branch, and `e<i><j>` an edge from i to j 

### variants
```yml
{{#include git-animate.yml:variants}}
```

we define here the colors for nodes and edges, using classes. The syntax `{tag}` will allow to replace the tag with its value, for each frame

### frames

we show here only two frames

```yml
{{#include git-animate.yml:frames}}
```

- the title is an html element
- toggles is a list of tag that will change value. Tags not present here will keep the value of the previous frame.


