from . import sort_search
import fontconfig
import argparse


def main():
    parser = argparse.ArgumentParser(description="Another font finder")
    parser.add_argument(
        "search_terms", metavar="search term", nargs="+", help="keywords to search for"
    )

    args = parser.parse_args()
    font_paths = fontconfig.query()
    get_score = sort_search(args.search_terms)
    best_path = max(font_paths, key=get_score)

    font = fontconfig.FcFont(best_path)
    print(f"Found: \x1b[38;5;82m{list(font.family.values())[0] or ''}\x1b[m at \x1b[4m{best_path}")

main()