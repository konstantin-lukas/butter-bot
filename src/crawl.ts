import { JSDOM } from "jsdom";
import { writeFile, readFileSync } from "fs";
import { User, TextChannel } from "discord.js";

export async function crawlDBDCodes(admin: User, channel: TextChannel) {
    const sources = [
        {
            url: "https://www.dexerto.com/dead-by-daylight/how-to-redeem-dead-by-deadlight-codes-1664016/",
            separator: ' – '
        },
        {
            url: "https://www.rockpapershotgun.com/dead-by-daylight-codes-list",
            separator: ': '
        },
        {
            url: "https://www.pcgamesn.com/dead-by-daylight/dbd-codes",
            separator: ' – '
        }
    ];
    const re = new RegExp(
        "[\u002D\u058A\u05BE\u1400\u1806\u2010\-\u2015\u2E17" +
        "\u2E1A\u2E3A\u2E3B\u2E40\u301C\u3030\u30A0\uFE31\uFE32\uFE58\uFE63\uFF0D]", "g")
    const responses = await Promise.all(sources.map(s => fetch(s.url)));
    const html = await Promise.all(responses.map(r => r.text()));

    let codes: {
        code: string,
        description: string
    }[] = [];
    const newCodes: {
        code: string,
        description: string
    }[] = [];

    try {
        let data = readFileSync("dbd_codes.json", "utf8");
        if (data) {
            const json = JSON.parse(data);
            if (json) codes = json;
        }
    } catch (e) {
        await admin.send("An error occurred trying to read exiting DBD codes:```\n" + e + "\n```");
    }

    for (let i = 0; i < html.length; i++) {
        const dom = new JSDOM(html[i]);
        const ul = dom.window.document.querySelector('h2 ~ ul');
        const listElements = ul?.querySelectorAll('li');
        if (listElements) {
            for (const li of listElements) {
                const textContent = li.textContent;
                const code = textContent?.split(sources[i].separator)[0]?.trim();
                const description = textContent?.split(sources[i].separator)[1]?.trim();
                if (code &&
                    description &&
                    !code?.match(re) &&
                    !description?.match(re) &&
                    !codes.some(x => x.code === code)
                ) {
                    codes.push({code, description});
                    newCodes.push({code, description});
                }
            }
        }

    }

    if (newCodes.length > 0) {
        writeFile("dbd_codes.json", JSON.stringify(codes), "utf8", async (e) => {
            if (e) {
                await admin.send("An error occurred trying to write exiting DBD codes to file:```\n" + e + "\n```");
            } else {
                let message = "I found some fresh new codes since my last crawl!\n\n";
                codes.forEach(x => {
                    message += `- \`${x.code}\`: ${x.description} (${new Date().toISOString().split('T')[0]})\n`
                });
                await channel.send(message);
            }
        });
    }
}