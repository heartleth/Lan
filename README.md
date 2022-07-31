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