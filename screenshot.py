import mss

def get_screenshots_bytes():
    with mss.mss() as sct:
        monitor = sct.monitors[0]
        sct_img = sct.grab(monitor)
        png = mss.tools.to_png(sct_img.rgb, sct_img.size)
        return png


if __name__ == "__main__":
    print(get_screenshots_bytes())
