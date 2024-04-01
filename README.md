# LogicLoom

A user can specify a rule according to which code should reacts . It would support a json as input as well as you can pass a yaml file also in input. 

We would provide support of  Rest API as well as sdk. 

# Description
Rule recevies as a generic input with like []<key: string , value: any> and generate a output
We can group rule on basis of their type which can be taken as a input from user and all rule of that group would be applied to input 
Each rule should have some condition and a rule should be applied if these conditions evalauate true 

# Rule 
`name: Rule Name `

The name of rule it should be unique 

`chain: conditionType`

default chain is `AND`

`conditions`

List of condition a user can pass 
  Each single condition has input path seperated by a dot 
  and functions property contains a set  of function which evaluate value specified in input path
  we ncan have list conditions, those which refer not to single field values but to lists of values. 
  we can also have constraint with list condition 
  We can also have aggregate condition 


 `Output` 
 When evaluating a set of rules, the engine combines the output of all the rules which succeeded into a single map and
returns it

#feature update




