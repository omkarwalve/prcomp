# WebSchema
```
|1.|2.|3.                                  |4.|5.                                     |
 S1 :: { base = (Class,product-container) } => [ name = (Attr,aria-type,product-name),
                                                 price = (Class,product-price),
                                                 rating = (Class,product-rating),
                                                 url = (Name,a)
                                               ];
```

1. **Branch**(`S1`) : The process branch.
2. **Process Seperator**(`::`) : Seperator to seperate name of branch from its process definition.
3. **Base Node**(`{ base = (,) }`): The basal node to start parsing through.  
   
   > - `{}=>[];` Indicates `Parents with Children` type `Schema`.
   > 
   > - `[];` Indicates `Direct Children` type `Schema`.
