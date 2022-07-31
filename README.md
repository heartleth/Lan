# LAN 표기법

    PART main
        RULE FizzBuzz $FizzBuzz( Fizz Buzz )
    END

    Part FizzBuzz
        # :0 | Fizz
        # :1 | Buzz

        IF 0 Fizz
            RULE Fizz Fizz-Fizz
        END

        IF 0 Buzz
            RULE Buzz Buzz-Buzz
        END
    END
    
## HQ9+

    PART main
        RULE H H-HelloWorld
        RULE Q Q-Quine
        RULE 9 9-Nine
        RULE + +-Plus
    END
