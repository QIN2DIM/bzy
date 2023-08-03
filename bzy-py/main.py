# -*- coding: utf-8 -*-
# Time       : 2023/8/3 22:52
# Author     : QIN2DIM
# Github     : https://github.com/QIN2DIM
# Description:
from __future__ import annotations

import asyncio
import logging
import os
import sys
import time
from dataclasses import dataclass, field
from datetime import datetime
from pathlib import Path
from typing import List, Tuple

import httpx
from httpx import AsyncClient

logging.basicConfig(
    level=logging.INFO, stream=sys.stdout, format="%(asctime)s - %(levelname)s - %(message)s"
)

CDN_PREFIX = "https://dl.capoo.xyz/"
URL_TASKS = CDN_PREFIX + "https://github.com/QIN2DIM/bzy/releases/download/bzy-db/BenZiYunMining.txt"


@dataclass
class Project:
    root = Path(__file__).parent
    database = root.joinpath("database")

    img_dir = database.joinpath("backup")
    bzy_index = database.joinpath("BenZiYunMining.txt")

    def __post_init__(self):
        os.makedirs(self.img_dir, exist_ok=True)

    def pull_bzy_index(self):
        if self.bzy_index.exists():
            return
        logging.info(f"正在下载数据集 - url={URL_TASKS}")
        res = httpx.get(URL_TASKS)
        self.bzy_index.write_bytes(res.content)

    def load_bzy_index(self) -> List[str] | None:
        if not self.bzy_index.exists():
            self.pull_bzy_index()
        urls = self.bzy_index.read_text().split("\n")
        logging.info(f"读入 {len(urls)} 条待处理链接")
        return urls


@dataclass
class AshFramework:
    task_queue: list = field(default_factory=list)

    @classmethod
    def from_samples(cls, samples: List[Tuple[str, Path]]):
        return cls(task_queue=samples)

    @staticmethod
    async def worker(client: AsyncClient, context):
        (url, sp) = context
        res = await client.get(url)
        sp.write_bytes(res.content)

    async def async_run(self):
        async with AsyncClient() as client:
            task_list = [self.worker(client, context) for context in self.task_queue]
            await asyncio.gather(*task_list)


def get_samples(batch_size: int = 10):
    project = Project()

    urls = project.load_bzy_index()
    # random.shuffle(urls)

    dt = datetime.now().strftime("%Y-%m-%d %H%M%S")
    dp = project.img_dir.joinpath(dt)
    os.makedirs(dp, exist_ok=True)

    samples = []
    for url in urls[:batch_size]:
        fn = url.split("/")[-1]
        sp = dp.joinpath(fn)
        samples.append((url, sp))

    return samples


async def main():
    samples = get_samples(batch_size=20)

    start = time.time()

    af = AshFramework.from_samples(samples)
    await af.async_run()

    elapsed = time.time() - start
    logging.info(f"任务结束 - Time elapsed: {elapsed:.2f}s")


if __name__ == '__main__':
    asyncio.run(main(), debug=False)
