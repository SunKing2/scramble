module ScrambleForm exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import String exposing (..)
import String.Extra exposing (..)


main : Program Never Model Msg
main =
    beginnerProgram { model = model, view = view, update = update }


interleaveString : String -> String -> String
interleaveString string string2 =
    fromList (interleave (toList string) (toList string2))


interleave : List a -> List a -> List a
interleave list1 list2 =
    case list1 of
        [] ->
            list2

        x :: xs ->
            case list2 of
                [] ->
                    list1

                y :: ys ->
                    y :: x :: interleave xs ys


validateWordResponse : String -> String
validateWordResponse guess =
    let
        valid =
            List.member guess model.validWords
    in
    case valid of
        True ->
            "good"

        False ->
            "bad"


type alias ViewItem =
    String


type alias Grid =
    String


type alias GridRow =
    String


type alias Model =
    { letters : String
    , guess : String
    , guesses : List String
    , validWords : List String
    , gameMessage : String
    }


model : Model
model =
    { letters = "ABCDEFGHIJKLMNOP"
    , guess = ""
    , guesses = []
    , validWords = [ "chicken", "hen", "dog", "cow" ]
    , gameMessage = "this game will never end, but try it out anyway"
    }


type Msg
    = SetGuess String
    | AddGuess


update : Msg -> Model -> Model
update msg model =
    case msg of
        SetGuess guess ->
            { model | guess = guess }

        AddGuess ->
            { model | guesses = model.guess :: model.guesses, guess = "", gameMessage = validateWordResponse model.guess }


view : Model -> Html Msg
view model =
    div []
        [ Html.form [ id "scrambleform", onSubmit AddGuess ]
            [ div [ id "scrambleformcontainer" ]
                [ label [ for "chat" ]
                    [ span []
                        [ text "Scramble game in progress: " ]
                    ]
                , textarea [ attribute "cols" "80", id "chat", name "chat", attribute "rows" "7" ]
                    [ text ("Welcome to Scramble\n" ++ toString model.guesses ++ "\n" ++ model.gameMessage) ]
                ]
            , text "*** Get ready for a round of Scramble (90 Seconds) ***"
            , grid model.letters
            , div [ id "guessandbutton" ]
                [ input [ id "guess", name "guess", placeholder "Enter words here", onInput SetGuess, value model.guess ]
                    []
                , input [ id "guessbutton", name "submit", type_ "submit", value "Send" ]
                    []
                ]
            ]
        ]


grid : Grid -> Html Msg
grid letters =
    div [] (List.map gridRow (break 4 model.letters))


gridRow : GridRow -> Html Msg
gridRow letters =
    div []
        [ text (interleaveString letters "    ")
        ]


viewItem : ViewItem -> Html Msg
viewItem item =
    li [] [ text item ]
