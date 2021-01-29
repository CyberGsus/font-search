import itertools
import string

import Levenshtein as levenshtein 


delimeters = string.whitespace + string.punctuation


# sort fonts based on search query:
#    key: compute the difference from the string
#    first: both should be ignore cased.
#    second: i will iterate through the most common matches.
#    each 'match' that it gets will be divided by the index (1, 2, 3..) this way
#    the most important letters will always be there.

# it adds to searching but it is not performant (this could probably be done in C directly)
def unword(s: str):

    last = ""
    alpha = True

    for c in s:
        if c in delimeters:
            if last:
                yield last
            last = ""
            continue

        if (alpha and (c.isupper() or c.isnumeric())) or (
            not alpha and c.isalpha()
        ):
            if last:
                yield last
            last = c
            alpha = c.isalpha()
            continue

        last += c

    if last:
        yield last


# def unword(s: str):
#     return re.split(r"\W", s)


def sort_search(search_terms: list[str]):
    search_terms = [*map(str.lower, search_terms)]

    def process_key(k: str):
        return k.strip().split("/")[-1].partition(".")[0]


    def ratio_levensthein(a: str, b: str):
        return levenshtein.ratio(a, b)

    def get_ratio(i: str, a: str, b: str):
        return ratio_levensthein(b, a) / i

    # def ratio_levensthein(i: str, a: str, b: str):

    def get_value(k: str):
        k = process_key(k)
        terms = map(str.lower, unword(k))
        return sum(
            map(
                lambda k: get_ratio(*k[0], k[1]),
                itertools.product(enumerate(terms, 1), search_terms),
            )
        )

    return get_value
