import random
import subprocess
import sys
import time
from urllib.parse import urlencode

import requests
import youtube_dl
from biliup import config

from ..engine.decorators import Plugin
from ..plugins import BatchCheckBase, match1
from ..engine.download import DownloadBase
from . import logger

VALID_URL_BASE = r'(?:https?://)?(?:(?:www|go|m)\.)?twitch\.tv/(?P<id>[0-9_a-zA-Z]+)'
_OPERATION_HASHES = {
    'CollectionSideBar': '27111f1b382effad0b6def325caef1909c733fe6a4fbabf54f8d491ef2cf2f14',
    'FilterableVideoTower_Videos': 'a937f1d22e269e39a03b509f65a7490f9fc247d7f83d6ac1421523e3b68042cb',
    'ClipsCards__User': 'b73ad2bfaecfd30a9e6c28fada15bd97032c83ec77a0440766a56fe0bd632777',
    'ChannelCollectionsContent': '07e3691a1bad77a36aba590c351180439a40baefc1c275356f40fc7082419a84',
    'StreamMetadata': '1c719a40e481453e5c48d9bb585d971b8b372f8ebb105b17076722264dfa5b3e',
    'ComscoreStreamingQuery': 'e1edae8122517d013405f237ffcc124515dc6ded82480a88daef69c83b53ac01',
    'VideoPreviewOverlay': '3006e77e51b128d838fa4e835723ca4dc9a05c5efd4466c1085215c6e437e65c',
}
_CLIENT_ID = 'kimne78kx3ncx6brgo4mv6wki5h1ko'


@Plugin.download(regexp=VALID_URL_BASE)
class Twitch(DownloadBase):
    def __init__(self, fname, url, suffix='flv'):
        DownloadBase.__init__(self, fname, url, suffix=suffix)

    def check_stream(self):
        # print(entry['extractor_key'], entry['id'], )
        # {'outtmpl': filename}
        with youtube_dl.YoutubeDL({'download_archive': 'archive.txt'}) as ydl:
            info = ydl.extract_info(self.url, download=False)
            # ydl.download(["https://www.twitch.tv/tsm_imperialhal/videos?filter=archives&sort=time"])
            # print(info['_type'])
            # print(info)
            for entry in info['entries']:
                if ydl.in_download_archive(entry):
                    continue
                self.raw_stream_url = entry['url']
                ydl.record_download_archive(entry)
                return True

    def download(self, filename):
        args = ['ffmpeg', '-y','-i', self.raw_stream_url,
                '-segment_time', f"{config.get('segment_time') if config.get('segment_time') else '00:50:00'}",
                '-f', 'segment',
                f'{self.fname} {time.strftime("%Y-%m-%d %H_%M_%S", time.localtime())} part-%03d.{self.suffix}']
        proc = subprocess.Popen(args, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        try:
            with proc.stdout as stdout:
                for line in iter(stdout.readline, b''):  # b'\n'-separated lines
                    print(line.decode(), end='', file=sys.stderr)
                    logger.debug(line.decode().rstrip())
            retval = proc.wait()
        except KeyboardInterrupt:
            if sys.platform != 'win32':
                proc.communicate(b'q')
            raise
        return retval

    class BatchCheck(BatchCheckBase):
        def __init__(self, urls):
            BatchCheckBase.__init__(self, pattern_id=VALID_URL_BASE, urls=urls)

        def check(self):
            with youtube_dl.YoutubeDL({'download_archive': 'archive.txt'}) as ydl:
                # ydl.download(["https://www.twitch.tv/tsm_imperialhal/videos?filter=archives&sort=time"])
                # print(info['_type'])
                # print(info)
                for channel_name, url in self.not_live():
                    info = ydl.extract_info(url, download=False)
                    for entry in info['entries']:
                        if ydl.in_download_archive(entry):
                            continue
                        yield url
                        # ydl.record_download_archive(entry)
                    # time.sleep(100)

        def not_live(self):
            gql = self.get_streamer()
            i = -1
            for data in gql:
                i += 1
                user = data['data'].get('user')
                if not user:
                    continue
                stream = user['stream']
                if not stream:
                    yield self.usr_list[i], self.usr_dict.get(self.usr_list[i])

        def get_streamer(self):
            ops = []
            for channel_name in self.usr_list:
                op = {
                    'operationName': 'StreamMetadata',
                    'variables': {'channelLogin': channel_name.lower()}
                }
                op['extensions'] = {
                    'persistedQuery': {
                        'version': 1,
                        'sha256Hash': _OPERATION_HASHES[op['operationName']],
                    }
                }
                ops.append(op)
            gql = requests.post(
                'https://gql.twitch.tv/gql',
                json=ops,
                headers={
                    'Content-Type': 'text/plain;charset=UTF-8',
                    'Client-ID': _CLIENT_ID,
                }, timeout=15)
            gql.close()
            return gql.json()