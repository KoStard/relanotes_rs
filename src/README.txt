So what we have...


Groups
- Some collection of nodes - describing one big concept
- Each group has it's own namespace - so in multiple groups same name can appear multiple times
- Groups have subgroups (so the Medicine is group and anatomy is subgroup)
- We make groups generally isolated (but links across groups are allowed, yet)

SubGroup
- these can have multiple links between each other

Nodes
- types
    - regular - the name will be unique - the main type, so can have children of any type
    - sticky_notes - just have some information about - can have children only of type sticky_notes
    - inherited - has a unique path - behaves like a regular node, because it IS a continuation of the regular node
        - preserve the path inside the GraphNode
        - update_path method - call update_path recursively on all child inherited nodes
    - symlinks - only for regular/inherited nodes

Unique Path:
- for inherited nodes
- a path from the nearest regular node among parents to current one
- is unique in the group namespace

Relative Path:
- currently only for sticky_notes

Path = Unique Path + Relative Path

Search
- as command
- for referencing inside description

Commands:
- change view mode
- start examination
- exit

Views:
- standard - showing only children of current node and the node content
- tree-like - from root or just a subtree of subgroup
- all-in-one

Examination:
- flashcards

Key bindings:
- Ctrl+P
- Ctrl+Shift+P
- Ctrl+Backspace
- Ctrl+Shift+Backspace - recursive delete?
- Ctrl+Enter

######################################################################
Questions:
- How will the start-up happen? What to load?
    - If we load whole group, then in the future it can take a long time...
    - If we load a subgroup, then global searching will be harder, because we don't have paths of nodes from other
    subgroups... For now this will be selected and global search will allow searching only by direct names (not with
    paths)... This is a restriction, but will make the design much more clear...

- How will we show the nodes tree? Not the GUI part itself, but the idea...
    - We can show all nodes, but with some sort of hierarchy, but this will make much harder to see the list of
    immediate children of the given node... This will make much harder the process of adding/removing/editing nodes
    - Maybe we can show only the children of the opened node - this will work, but the usage can be harder,
    because the user don't see all nodes. Currently I prefer this one...
    - Show just a tree of connected nodes, but this is not practical, because we won't be able to understand the content

- How will we show the node content and allow the user to edit it?
    - Maybe side-by-side with the children? When a child is focused and you press a side arrow/tab then you'll start
    editing the parent node, when you press Ctrl+Enter then you'll close the editing mode. This will allow you to see
    the node content when you are opening the node.

- How will we handle user input?
    - Ctrl+P for path search and Ctrl+Shift+P for commands like in VSCode?

- What front-end to use?
    - Text-based - this won't allow us getting modern design, but anyway this should work + we'll get a better performance
    - With WebView we'll get modern design, but the performance is unknown yet... Maybe we can try to include WASM files...
    - Druid?

- Will we allow children to be located in other subgroups? We need this because the regular nodes names are unique,
but there can be some information related to the given node in another subgroup...
    - We can just add a possibility to add children in other subgroups (by selecting the subgroup of the child or
    by adding possibility to select the parent from any subgroup root)..
    - Add nodes like symlinks, that will have a reference to a node located in another subgroup... Only into
    the root, because this would allow creation of nodes with multiple owners... But the symlink nodes already have
    their 'owners' - Currently using this solution

    * The problem here is that we have to get the node path somehow

- How will we handle node referencing in the description?
    - When user enters # start inline search

- How will we handle node changes?
    - Maybe add save method like in Django and save the current state - maybe make this private and add methods for
    handling changes of each field separately, because these changes can lead to some problems (if we change the tree
    structure).

- How will the search work?
    - I think we'll need something like in VSCode's search, so that we can write some characters from the grandparent's
    name and some from the target node and we'll find the node...
        - use paths - can lead to some confusion if the query tries to find a regular node but has some symbols from a
        parent node
        - How will we handle symlinks here?
            - if the symlink's target is a regular node, then everything is simple
            - if the target is inherited node, then this will become much harder, because we can have multiple symlinks
            to different inherited nodes with same name (but different paths)... Maybe we have to create a SQL function
            that will generate the symlink path... We can't save it in the DB, because we'll need to update it with
            every change - anyway this can lead to problems if we somehow edit multiple subgroups in parallel, because
            the path can be changed...
    - Handle IDs too


- How will the development happen?
    - Currently I have relanotes_rs and relanotes_webview repos, but if I want to merge the development of the core and
    the GUI then maybe I have to merge these repos...
    - Maybe use git submodules during development?
    - Maybe just add cargo dependencies with local path?
        - Maybe with git too - https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-path-dependencies

- How will we handle the node types?
    - Considering the current list of node types we can save all nodes in one table, but have multiple GraphNode enum
    variants or multiple structs with implementing same trait to allow saving in the HashMap (this will make it harder to
    determine the node type)!
    - Or maybe we can save them separately, but this will create big problems while saving and getting nodes using HashMap

- How will we handle examination?
    - Modes
        - Maybe add flashcards and learn them layer by layer going deeper in the tree
    - We have to save the progress
        - Maybe save in a new table

- How will back-end communicate with front-end?
    * We have to understand the possible program states
        * Just opened a node or looking at the root in standard view - this is easy, the list won't be so big, so we can
        render everything from scratch and get whole data from back-end
        * Searching - the results has to be rendered each time from scratch, because we can't know anything about results,
        but this will be easy too
        * Tree-like view - this will be heavy to re-render, because we include here whole or a big part of the subgroup,
        but this is just a demo view, so we won't need to re-render this frequently
        * All-in-one - We have to be careful with this, because this can worsen the performance if we render whole
        subgroup each time
    - Just send a JSON representative of given nodes - we'll check the performance

- Will you be able to load just a part of the subgroup?
    - Maybe not, because we'll have non-symlink nodes that have parents not in the current scope!

- What will happen when user tries to remove group or subgroup?
    - Maybe we can show some warning message but allow

- How will the user create new nodes?
    - Maybe add button to add new nodes, but add key binding too and maybe allow changing the new node type using
    only key bindings

- What will happen when user tries to remove a node that has children
    - Maybe that depends on what are the node types - maybe allow to remove the node if it has only sticky_notes
    - Or delete every connected sticky_notes or inherited nodes
    - Maybe allow moving child nodes to the parent and then remove the node

- How will the user delete nodes?
    - Maybe we can allow the user to delete nodes without opening it - maybe with key bindings
    - What about recursive deletes?

- Will the user be able to move the node to other parents or even groups?

############################################################
Tasks

- add subgroups
- load whole subgroup
- add node types and show them in graph nodes
- add logic for serializing the nodes - don't include their children or parents, just them
- add basic GUI
    - select group
    - select subgroup
    - open nodes
    - change the content of the node
    - see children
    - see the path to current node
    - add/delete nodes
