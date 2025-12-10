import json

with open("coverage.json") as f:
    data = json.load(f)

def bgcolor(percent):
    if percent <= 35:
        return "#ff3826"
    elif 36 <= percent <= 60:
        return "#ff8426"
    elif 61 <= percent <= 79:
        return "#ffd046"
    elif 80 <= percent <= 90:
        return "#bdff3a"
    else:
        return "#4eff3a"

html_lines = []
html_lines.append("<table>")
html_lines.append("<tr><th>Filename</th><th>Function Coverage</th><th>Line Coverage</th><th>Region Coverage</th><th>Branch Coverage</th></tr>")

for item in data["data"]:
    for file_cov in item["files"]:
        filename = file_cov["filename"]
        summary = file_cov["summary"]

        func_cov_val = summary["functions"]["percent"]
        line_cov_val = summary["lines"]["percent"]
        region_cov_val = summary["regions"]["percent"]
        branch_cov_val = summary["branches"]["percent"]

        func_cov = f'<td style="background-color:{bgcolor(func_cov_val)}">{func_cov_val:.2f}% ({summary["functions"]["covered"]}/{summary["functions"]["count"]})</td>'
        line_cov = f'<td style="background-color:{bgcolor(line_cov_val)}">{line_cov_val:.2f}% ({summary["lines"]["covered"]}/{summary["lines"]["count"]})</td>'
        region_cov = f'<td style="background-color:{bgcolor(region_cov_val)}">{region_cov_val:.2f}% ({summary["regions"]["covered"]}/{summary["regions"]["count"]})</td>'
        branch_cov = f'<td style="background-color:{bgcolor(branch_cov_val)}">{branch_cov_val:.2f}% ({summary["branches"]["covered"]}/{summary["branches"]["count"]})</td>'

        html_lines.append(f"<tr><td>{filename}</td>{func_cov}{line_cov}{region_cov}{branch_cov}</tr>")

html_lines.append("</table>")

with open("COVERAGE.md", "w") as f:
    f.write("\n".join(html_lines))

print("COVERAGE.md generated with full HTML heatmap table!")
