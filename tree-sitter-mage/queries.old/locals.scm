;; Variable declarations
(variable_declaration
  name: (identifier) @definition.var)

;; Parameter definitions
(parameter_list
  (identifier) @definition.parameter)

;; Function declarations
(function_declaration
  name: (identifier) @definition.function)

;; References
(identifier) @reference 