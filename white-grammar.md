# White-Lang Grammar

- program : statements
- statements : statement ";" | statements statement ";" | nothing
- statement : for_statement | while_statement | function_def_statement | 
funciton_call_statement | if_statement | assignment_statement
- for_statement : "for" "(" {identifier "in" (list_literal | integer_literal ".." integer_literal) 
} | {iterator} ")" "{" statements "}" 
- while_statement : "while" "(" {boolean_literal | comparison_expression} ")" "{" statements "}"
- function_def_statement : { access } identifier "(" args ")" { "->" type } "{" statements "}"
- access : "public" | "private" | "protected"
- args : expression | args expression | nothing
- function_call_statement : function_call_expression
- assignment_statement : "let" "identifier" { ":" type } "=" expression
- if_statement : "if" "(" {boolean_literal | comparison_expression} ")" "{" statements "}" { else "{" statements "}"
- expression : boolean_literal | comparison_expression | float_literal | integer_literal | ...