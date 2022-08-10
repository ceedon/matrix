module Matrix exposing (main)

--import Element.Border     as        Border
--import Element.Font       as        Font

import Browser
import Dict exposing (Dict)
import Element exposing (..)
import Element.Background as Background
import Element.Events exposing (onClick)
import Element.Region exposing (description)
import Html exposing (Html)
import Tuple exposing (first, second)
import Validate exposing (fromErrors, validate)


gridWidth : Int
gridWidth =
    8


gridHeight : Int
gridHeight =
    8



-- Model


type alias Model =
    { grid : Grid
    , cellsClicked : List Pos
    }


type alias Grid =
    Dict Pos CellColor


type alias Pos =
    ( Int, Int )


type CellColor
    = Black
    | Green
    | Red
    | Orange


ok : Model
ok =
    Model Dict.empty []



-- Init


init : () -> ( Model, Cmd Msg )
init _ =
    ( ok, Cmd.none )



-- View


view : Model -> Html Msg
view =
    printGrid


drawMatrix _ =
    Element.layout
        []
    <|
        text "Hello, World!"


printGrid : Model -> Html Msg
printGrid model =
    let
        grid =
            model.grid

        xs : List Int
        xs =
            List.range 1 gridWidth

        ys : List Int
        ys =
            List.range 1 gridHeight

        toRow : Int -> List Pos
        toRow y =
            List.map (\x -> ( x, y )) xs
    in
    layout [] <|
        row []
            [ column []
                (List.map
                    (\y ->
                        drawCellRow grid (toRow y)
                    )
                    ys
                )
            ]


drawCell : Grid -> Pos -> Element Msg
drawCell _ p =
    el
        [ onClick <| ClickedCell p
        , moveDown 256
        , moveRight ((toFloat gridWidth + 1) * 128)
        ]
    <|
        image
            [ moveDown ((64 * toFloat (first p)) - (320 * toFloat (second p)))
            , moveLeft ((128 * toFloat (first p)) + (128 * toFloat (second p)))
            ]
            { src = "./img/block-barren.png"
            , description = "Stolen block"
            }


drawCellRow : Grid -> List Pos -> Element Msg
drawCellRow b ps =
    row [] (List.map (drawCell b) ps)



-- Update


type Msg
    = ClickedCell Pos
    | ClickedDraw Grid


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        ClickedCell p ->
            ( { model | cellsClicked = p :: model.cellsClicked }
            , Cmd.none
            )

        ClickedDraw g ->
            ( model, Cmd.none )



-- Subscriptions


subscriptions : Model -> Sub Msg
subscriptions _ =
    Sub.none



-- Main


main : Program () Model Msg
main =
    Browser.element
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }