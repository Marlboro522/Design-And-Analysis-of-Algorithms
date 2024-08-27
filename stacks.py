#Stacks using linked list. 
class Node:
    def __init__(self,data) -> None:
        self.data = data
        self.next = None
class Stack:
    def __init__(self) -> None:
        self.top=None
    def push(self,data):
        new_node=Node(data)
        if self.top:
            new_node.next=self.top
        self.top=new_node
    def pop(self):
        if self.top is None:
            return None
        else:
            popped_node=self.top
            self.top=self.top.next
            popped_node.next=None
            return popped_node.data
    def peek(self):
        if self.top is None:
            return None
        return self.top.data
    
#We can also use the LifoQueue class from the python's queue module. Behave like a stack.   

import queue

my_stack = queue.LifoQueue(maxsize=0) # maxsize 0 gives infinite stack size
print("Stack size of my_stack is: ", my_stack.qsize())

        
