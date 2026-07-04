#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

sudo pacman -Syu --needed --noconfirm base base-devel git

if command -v yay &>/dev/null; then
    echo "Yay is installed."
else
    echo "Installing yay"
    cd ~
    git clone https://aur.archlinux.org/yay.git
    cd yay
    makepkg -i
    cd ~
    rm -rf yay
fi

echo "Installing gum for a better experience"
yay -S --needed --noconfirm gum

if gum confirm "Install a bunch of essential packages? (required)"; then
    yay -S --needed --noconfirm bluez bluez-utils brightnessctl htop man-db nano niri noto-fonts noto-fonts-cjk noto-fonts-emoji pipewire-audio pipewire-pulse playerctl ttf-dejavu ttf-jetbrains-mono ttf-jetbrains-mono-nerd ttf-liberation ttf-nerd-fonts-symbols-mono xwayland-satellite wget adw-gtk-theme nautilus
    gsettings set org.gnome.desktop.interface gtk-theme 'adw-gtk3-dark' && gsettings set org.gnome.desktop.interface color-scheme 'prefer-dark'
    cp -r ~/configurator/.config/niri ~/.config
else
    exit
fi

if gum confirm "Install SDDM?"; then
    yay -S --needed --noconfirm sddm
    sudo systemctl enable sddm.service
fi

if gum confirm "Theme SDDM with sddm-astronaut-theme?"; then
    echo "Theming sddm ..."
    bash -c "$(curl -fsSL https://raw.githubusercontent.com/keyitdev/sddm-astronaut-theme/master/setup.sh)"
fi

if gum confirm "Install a lock screen?"; then
    yay -S --needed --noconfirm hypridle hyprlock
    cp -r $SCRIPT_DIR/.config/hypr ~/.config
    cp -r $SCRIPT_DIR/.config/hyprlock ~/.config
    echo "IMPORTANT: Choose your lock screen theme in ~/.config/hyprhyprlock.conf"
fi

if gum confirm "Install system settings apps? (highly recommended)"; then
    yay -S --needed --noconfirm blackbox-terminal bm-sidebar junction mission-center nirimod-git pwvucontrol walker-bin elephant-desktopapplications elephant-providerlist adw-network
    cargo install --path $SCRIPT_DIR/.config/dashboard-gtk
fi

if gum confirm "Setup zsh and oh-my-zsh?"; then
    yay -S --needed --noconfirm zsh
    cp $SCRIPT_DIR/.zshrc ~
    git clone --depth=1 https://github.com/romkatv/powerlevel10k.git "${ZSH_CUSTOM:-$HOME/.oh-my-zsh/custom}/themes/powerlevel10k"
    git clone https://github.com/zsh-users/zsh-autosuggestions ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-autosuggestions
    git clone https://github.com/zsh-users/zsh-syntax-highlighting.git ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-syntax-highlighting
fi

if gum confirm "Install a notification daemon (swaync)?"; then
    yay -S --needed --noconfirm swaync
    cp $SCRIPT_DIR/.config/swaync ~/.config
fi

if gum confirm "Install other cool gtk apps?"; then
    yay -S --needed --noconfirm apostrophe blanket cine gfeeds-git gnome-font-viewer gnome-mahjongg gnome-shell-pomodoro gnome-weather impression keypunch-git loupe shortwave switcheroo
fi

if gum confirm "Done! Reboot?"; then
    reboot
fi
