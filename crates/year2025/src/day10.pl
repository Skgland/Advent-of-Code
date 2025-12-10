:- use_module(library(between)).
:- use_module(library(clpz)).
:- use_module(library(iso_ext)).
:- use_module(library(lambda)).
:- use_module(library(lists)).

:- initialization(assertz(clpz:monotonic)).

apply_button(Mask, Button, Presses, Count) :- #Flag #= popcount(Button /\ Mask), #Count #= #Presses * Flag.

constrain_joltages([], _, _, _).
constrain_joltages([J|Js], Idx, Vars, Bs) :- 
    Mask is 1 << Idx,
    maplist(
        apply_button(Mask), 
        Bs, 
        Vars,
        Counts
    ),
    sum(Counts, #=, J),
    Idx1 = Idx + 1 ,
    constrain_joltages(Js, Idx1, Vars, Bs).

solve_machine(Buttons, Joltages, N) :-
    length(Buttons, NumButtons),
    length(Vars, NumButtons),
    Vars ins 0..sup,
    N in 0..sup,
    sum(Vars, #=, #N),
    constrain_joltages(Joltages, 0, Vars, Buttons),
    once(labeling([min(N)],[N|Vars])),
    write(N), nl.

part2(Machines, N) :- 
    maplist(
        \machine(Buttons, Joltages)^M^solve_machine(Buttons, Joltages, M),
        Machines,
        Results
    ),
    sum_list(Results, N).
