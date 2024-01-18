# LAN 표기법

    PART NaturalNumber
        # :n | explaination of nth parameter (zero-based)
        # comment
        
        # production rule
        RULE number     *numeric ?$NaturalNumber
        
        # *category : category's vocabulary in dictionary file
        # $NonTerminalSymbol
        # ?*category, ?$Sym : can be omitted
        # [:n=foo]*category, [:n=foo]$Sym : only activated when nth parameter is same with 'foo'. 
        # $Symbol( param1 param2 ) : nonterminal symbol with parameters, whitespace separated, default "0"
        
        # if, unless statement
        IF :parameter value
            RULE ... ......
        END
        IF :parameter ~ match_value1 match_value2 match_value3
            RULE ... ......
        END
        UNLESS :parameter value
            RULE ... ......
        END
        
        # grammer alias
        SET zerozero 0-terminal_symbol_zero 0-terminal_symbol_zero
        
        RULE hundred    1-one %zerozero
    END

[GNE](https://github.com/heartleth/GNE)에 따로 구현되어 있는 코드가 더 잘 작동합니다.
