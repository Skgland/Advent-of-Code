:- use_module(library(arithmetic)).
:- use_module(library(between)).
:- use_module(library(clpz)).
:- use_module(library(dcgs)).
:- use_module(library(iso_ext)).
:- use_module(library(lambda)).
:- use_module(library(lists)).
:- use_module(library(simplex)).


apply_button(Mask, Button, Flag) :- N is Button /\ Mask, popcount(N, Flag).

constrain_joltage(J, JIdx, Buttons) -->
    { 
        Mask is 1 << JIdx,
        findall(b(BIdx), (nth0(BIdx, Buttons, Button), apply_button(Mask, Button, 1)), Parts)
    },
    constraint(Parts = J).

constrain_joltages([], _, _, S, S).
constrain_joltages([J|Js], Idx, Bs) -->
    constrain_joltage(J, Idx, Bs),
    { Idx1 is Idx + 1 },
    constrain_joltages(Js, Idx1, Bs).

constrain_buttons([],_,S, S).
constrain_buttons([_|Bs],Idx) -->
    {Idx1 is Idx + 1},
    constraint([b(Idx)] >= 0),
	constraint(integral(b(Idx))),
    constrain_buttons(Bs, Idx1).

apply_constraints(Joltages, Buttons) -->
    constrain_joltages(Joltages, 0, Buttons),
    constrain_buttons(Buttons, 0),
    { findall(b(BIdx), nth0(BIdx, Buttons, _), Variables) },
    minimize(Variables),
    objective.

solve_machine(Buttons, Joltages, N) :- gen_state(S0), apply_constraints(Joltages, Buttons, S0, N), write(presses(N)), nl.

part2(Machines, N) :- 
    maplist(
        \machine(Buttons, Joltages)^M^solve_machine(Buttons, Joltages, M),
        Machines,
        Results
    ),
    sum_list(Results, N).